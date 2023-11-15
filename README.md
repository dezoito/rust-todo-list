# Rust Todo List

This is a simple command line todo list app written in Rust as an exercise with specific goals in mind:

- Use a database to persist todos.
- Minimize the use of `unwrap()` and `expect()` calls.
- Include relevant working test cases, even if they are not 100% necessary.

The idea for this project was inspired by [Top 15 Rust Projects To Elevate Your Skills](https://zerotomastery.io/blog/rust-practice-projects/) and the suggested [CLI todo app](https://github.com/sioodmy/todo).

If you are new to Rust and would like a better understanding on some of the coding and design decisions for this project, please check my [Rust Todo SQL Example Application](https://dezoito.github.io/2023/11/01/rust-todo-example-application.html) article!

## Available commands

```
Available commands:

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
        Sorts completed and pending tasks

    - reset
        Deletes all tasks

```

## Development commands:

```sh
# to run once
cargo run <command args>

# to run in watch mode
cargo watch -c -x "run -- <command args>"

# to run tests in watch mode
cargo watch -c -x test
```

## Notes:

You need to install [cargo-watch](https://crates.io/crates/cargo-watch).

```sh
cargo install cargo-watch
```

The executable will attempt to create a directory called `todo_db` in your `HOME` or `PROFILE` folder, depending on the OS you are using.

This directory is used to store the SQLITE database.

## Building

I use Linux, so building an executable means running

```sh
cargo build --release
```

There's also a convenience script called `./build.sh` that builds and moves the executable to `/usr/local/bin`

Cross-Compile for Windows:
Make sure you have the Rust target for Windows installed. You can do this with the following command:

```sh
rustup target add x86_64-pc-windows-gnu
```

Then, build your Rust project with the Windows target specified:

```sh
cargo build --release --target x86_64-pc-windows-gnu
```

After you build, copy the resulting executable into your `path` and run it:

```sh
todo list
```

## References:

- https://zerotomastery.io/blog/rust-practice-projects/
- https://github.com/sioodmy/todo
- https://tedspence.com/investigating-rust-with-sqlite-53d1f9a41112
