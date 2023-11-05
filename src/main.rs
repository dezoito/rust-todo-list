extern crate todo;
use std::env;
use todo::*;

#[allow(unused)] // Remove later
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("You need to pass some arguments!");
        help();
        std::process::exit(1);
    }

    let command = &args[1];
    let suffix = &args[2..].iter().cloned().collect::<Vec<_>>().join(" ");

    match command.as_str() {
        "add" => {}
        "list" => {}
        "toggle" => {}
        "reset" => {}
        "rm" => {}
        "sort" => {}
        "help" | "--help" | "-h" | _ => help(),
    };
}
