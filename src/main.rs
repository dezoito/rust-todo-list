extern crate todo;
use console::style;
use dialoguer::Confirm;
use rusqlite::Result;
use std::env;

use todo::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Get a connection to the DB
    let conn = get_connection()?;

    if args.len() == 1 {
        help()?;
        std::process::exit(1);
    }

    let command = &args[1];
    let suffix = &args[2..].iter().cloned().collect::<Vec<_>>().join(" ");

    match command.as_str() {
        "add" => {
            if suffix.as_str().is_empty() {
                help()?;
                std::process::exit(1);
            } else {
                Todo::add(&conn, suffix.as_str())?;
            }
            Ok(())
        }
        "list" => {
            println!("TODO List (sorted by id):");
            let todos = Todo::list(&conn, false)?;
            Todo::print_list(todos)?;
            Ok(())
        }
        "toggle" => {
            if args.len() < 3 {
                help()?;
                std::process::exit(1);
            } else {
                let id = args[2].parse::<i32>().unwrap();
                Todo::toggle(&conn, id)?;
                println!("Toggled task with ID: {}", id);
            }
            Ok(())
        }
        "reset" => {
            let confirmation = Confirm::new()
                .with_prompt(
                    style("Do you want REALLY want to reset?")
                        .bright()
                        .red()
                        .to_string(),
                )
                .interact();

            match confirmation {
                Ok(c) => {
                    if c {
                        Todo::reset(&conn)?;
                        println!("Database reset. All tasks cleared.");
                    } else {
                        println!("Alright. No reset!");
                    }
                }
                Err(e) => {
                    eprintln!("{}", e)
                }
            }
            Ok(())
        }
        "rm" => {
            if args.len() < 3 {
                help()?;
                std::process::exit(1);
            } else {
                let id = args[2].parse::<i32>().unwrap();
                Todo::rm(&conn, id)?;
                println!("Removed task with ID: {}", id);
            }
            Ok(())
        }
        "sort" => {
            println!("TODO List (sorted by status):");
            let todos = Todo::list(&conn, true)?;
            Todo::print_list(todos)?;
            Ok(())
        }
        "help" | "--help" | "-h" | _ => help(),
    }?;
    Ok(())
}
