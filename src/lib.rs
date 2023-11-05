use console::style;

// Prints help with a list of commands and parameters
pub fn help() {
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
}
