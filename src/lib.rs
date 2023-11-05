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
    pub date_added: String,
    pub is_done: i32,
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

// Returns a connection, creating the database if needed
pub fn get_connection() -> Result<Connection> {
    let db_folder = get_home() + "/" + "todo_db/";
    let db_file_path = db_folder.clone() + "todo.sqlite";
    verify_db_path(&db_folder)?;
    let conn = Connection::open(db_file_path)?;
    verify_db(&conn)?;
    Ok(conn)
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
