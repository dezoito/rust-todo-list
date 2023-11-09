use console::style;
use rusqlite::{Connection, Result};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub date_added: String, // Using f64 for timestamp
    pub is_done: i32,
}

impl Todo {
    // Constructor for a new Todo instance
    pub fn new(id: i32, name: String, date_added: String, is_done: i32) -> Self {
        Todo {
            id,
            name,
            date_added,
            is_done,
        }
    }

    // Add a new todo to the database
    pub fn add(conn: &Connection, name: &str) -> Result<()> {
        conn.execute("INSERT INTO todo (name) VALUES (?)", &[name])?;
        Ok(())
    }

    // List all todo entries in the database
    pub fn list(conn: &Connection, sort_by_status: bool) -> Result<Vec<Todo>> {
        let sql = if sort_by_status {
            "SELECT * FROM todo ORDER BY is_done, id"
        } else {
            "SELECT * FROM todo ORDER BY id"
        };
        let mut stmt = conn.prepare(sql)?;
        let todo_iter = stmt.query_map((), |row| {
            Ok(Todo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }

    // Toggle the 'is_done' property of a Todo
    pub fn toggle(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("UPDATE todo SET is_done = 1 - is_done WHERE id = ?", &[&id])?;
        Ok(())
    }

    // Reset the database, clearing all entries
    pub fn reset(conn: &Connection) -> Result<()> {
        conn.execute("DELETE FROM todo", ())?;
        Ok(())
    }

    // Removes a task
    pub fn rm(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("DELETE FROM todo WHERE id = ?", &[&id])?;
        Ok(())
    }

    // Prints a list of todos objects
    pub fn print_list(todos: Vec<Todo>) -> Result<()> {
        for todo in todos {
            // Styles the string representing the status
            let status = if todo.is_done == 1 {
                style("Done").green()
            } else {
                style("Pending").red()
            };
            println!(
                "{:>4} | {:<44} {:<8} {}",
                style(todo.id).cyan().bright(),
                style(truncate_at(&todo.name, 44)).bright(),
                status,
                style(todo.date_added).dim(),
            );
        }
        Ok(())
    }
}

// Truncates an str and adds ellipsis if needed
pub fn truncate_at(input: &str, max: i32) -> String {
    let max_len: usize = max as usize;
    if input.len() > max_len {
        let truncated = &input[..(max_len - 3)];
        return format!("{}...", truncated);
    };

    input.to_string()
}

// Returns a connection, creating the database if needed
pub fn get_connection() -> Result<Connection> {
    let db_folder = get_home() + "/" + "todo_db/";
    let db_file_path = db_folder.clone() + "todo.sqlite";
    verify_db_path(&db_folder)?;
    let conn = Connection::open(db_file_path)?;
    verify_db(&conn)?;
    Ok(conn)
}

// Aux function that creates tables if they don't exist
pub fn verify_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
    	id	        INTEGER NOT NULL,
    	name	    TEXT NOT NULL,
    	date_added	REAL NOT NULL DEFAULT current_timestamp,
    	is_done	    NUMERIC NOT NULL DEFAULT 0,
    	    PRIMARY KEY(id AUTOINCREMENT)
    )",
        [], // no params for this query
    )?;
    Ok(())
}

// Aux function that creates the folder where the DB should be stored
// if it doesn't exist
pub fn verify_db_path(db_folder: &str) -> Result<()> {
    if !Path::new(db_folder).exists() {
        // Check if the folder doesn't exist
        match fs::create_dir(db_folder) {
            Ok(_) => println!("Folder '{}' created.", db_folder),
            Err(e) => eprintln!("Error creating folder: {}", e),
        }
    }
    Ok(())
}

// Prints help with a list of commands and parameters
pub fn help() -> Result<()> {
    let help_title = "\nAvailable commands:";
    let help_text = r#"    
        - add [TASK]
            Ads new task/s
            Example: todo add "Build a tree"

        - list
            Lists all tasks
            Example: todo list

        - toggle [ID]
            Toggles the status of a task (Done/Pending)
            Example: todo toggle 2
        
        - rm [ID]
            Removes a task
            Example: todo rm 4
        
        - sort
            Sorts completed and uncompleted tasks
        
        - reset
            Deletes all tasks
        "#;

    println!("{}", style(help_title).cyan().bright());
    println!("{}", style(help_text).green());
    Ok(())
}

// Get the user's home directory for each platform
fn get_home() -> String {
    let home_dir = match env::var("HOME") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            // Fallback for Windows and macOS
            if cfg!(target_os = "windows") {
                if let Some(userprofile) = env::var("USERPROFILE").ok() {
                    PathBuf::from(userprofile)
                } else if let Some(homedrive) = env::var("HOMEDRIVE").ok() {
                    let homepath = env::var("HOMEPATH").unwrap_or("".to_string());
                    PathBuf::from(format!("{}{}", homedrive, homepath))
                } else {
                    panic!("Could not determine the user's home directory.");
                }
            } else if cfg!(target_os = "macos") {
                let home = env::var("HOME").unwrap_or("".to_string());
                PathBuf::from(home)
            } else {
                panic!("Could not determine the user's home directory.");
            }
        }
    };

    // Convert the PathBuf to a &str
    match home_dir.to_str() {
        Some(home_str) => home_str.to_string(),
        None => panic!("Failed to convert home directory to a string."),
    }
}

/* -----------------------------------------------------------
 *    Tests
 * ---------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    // Creates a persistant in memory db connection
    // creates tables if necessary
    lazy_static! {
        static ref DATABASE_CONNECTION: Mutex<Connection> = {
            let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
            verify_db(&conn).expect("Cannot create tables");
            Mutex::new(conn)
        };
    }

    fn reset_db(conn: &Connection) -> Result<()> {
        conn.execute("DELETE FROM todo", ())?;
        Ok(())
    }

    fn contains_task(todos: &Vec<Todo>, target_name: &str) -> bool {
        for todo in todos {
            if todo.name == target_name {
                return true;
            }
        }
        false
    }

    #[test]
    fn test_add_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutex lock failed");
        reset_db(&conn).expect("Fucked up resetting the db");

        // Call the add function to add a todo
        let name = "Test Todo";
        Todo::add(&conn, name).expect("Failed to add todo");

        // Query the database to check if the todo was added
        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM todo WHERE name = ?")
            .expect("Failed to prepare statement");
        let count: i32 = stmt
            .query_row(&[name], |row| row.get(0))
            .expect("Failed to query database");

        assert_eq!(count, 1, "Todo was not added to the database");
    }

    #[test]
    fn test_list_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutex lock failed");
        reset_db(&conn).expect("Fucked up resetting the db");

        Todo::add(&conn, "Task 1").expect("Could not add todo");
        Todo::add(&conn, "Task 2").expect("Could not add todo");
        Todo::add(&conn, "Task 3").expect("Could not add todo");
        let todos = Todo::list(&conn, false).expect("Failed to list todo");

        assert_eq!(
            todos.len(),
            3,
            "Wrong number of todo items returned by list()"
        );
    }

    #[test]
    fn test_sort_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutex lock failed");
        reset_db(&conn).expect("Fucked up resetting the db");

        Todo::add(&conn, "Task 1").expect("Could not add todo");
        Todo::add(&conn, "Task 2").expect("Could not add todo");
        Todo::add(&conn, "Task 3").expect("Could not add todo");
        let todos = Todo::list(&conn, false).expect("Failed to list todo");
        // toggles the first entry
        Todo::toggle(&conn, todos[0].id).expect("Could not toggle first todo");

        // true means sorted by status
        let todos = Todo::list(&conn, true).expect("Failed to sort todos");

        assert_eq!(
            todos[2].name, "Task 1",
            "The todo marked as done was not the LAST one returned"
        );

        assert_eq!(
            todos.len(),
            3,
            "Wrong number of todo items returned by sort()"
        );
    }

    #[test]
    fn test_rm_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutex lock failed");
        reset_db(&conn).expect("Fucked up resetting the db");

        Todo::add(&conn, "Task 1").expect("Could not add todo");
        Todo::add(&conn, "Task 2").expect("Could not add todo");
        Todo::add(&conn, "Task 3").expect("Could not add todo");
        let todos = Todo::list(&conn, false).expect("Failed to list todo");
        // toggles the first entry
        Todo::rm(&conn, todos[0].id).expect("Could not remove first todo");

        // true means sorted by status
        let todos = Todo::list(&conn, false).expect("Failed to sort todos");
        dbg!(&todos);

        assert_eq!(
            todos.len(),
            2,
            "Wrong number of todo items returned by sort()"
        );

        assert_eq!(
            contains_task(&todos, "Task 1"),
            false,
            "Task 1 was not deleted!"
        );
    }

    #[test]
    fn test_toggle_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutex lock failed");
        reset_db(&conn).expect("Fucked up resetting the db");

        Todo::add(&conn, "Task 1").expect("Could not add todo");
        Todo::add(&conn, "Task 2").expect("Could not add todo");
        let todos = Todo::list(&conn, false).expect("Failed to list todo");
        // toggles the first entry
        Todo::toggle(&conn, todos[0].id).expect("Could not toggle first todo");

        // true means sorted by status
        let todos = Todo::list(&conn, false).expect("Failed to sort todos");
        dbg!(&todos);

        assert_eq!(
            todos.len(),
            2,
            "Wrong number of todo items returned by toggle()"
        );

        // True and False are stored as 0  or 1 in the db
        assert_eq!(todos[0].is_done, 1, "Task 1 was not toggled!");
    }

    #[test]
    fn test_reset_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutex lock failed");
        Todo::add(&conn, "Some task").expect("Could not add todo");
        Todo::reset(&conn).expect("Fucked up resetting the db");

        let todos = Todo::list(&conn, false).expect("Failed to list todo");

        assert_eq!(todos.len(), 0, "There are still todos left after reset()");

        assert_eq!(
            contains_task(&todos, "Task 1"),
            false,
            "Task 1 was not deleted!"
        );
    }
}
