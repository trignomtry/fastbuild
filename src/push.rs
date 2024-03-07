use std::fs::File;
use std::io::{self, Write};

pub fn push() {
    let mut user_input = String::new();

    println!("Commit Message");

    // Read user input with error handling
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {

            std::process::Command::new("git")
            .args(&["add", "."])
            .spawn()
            .expect("failed to spawn git add");
            std::process::Command::new("git")
            .args(&["commit", "-m", user_input.trim()])
            .spawn()
            .expect("failed to spawn git commit");
            std::process::Command::new("git")
            .args(&["push", "origin"])
            .spawn()
            .expect("failed to spawn git push");

            user_input.pop();
            write!(file, "type = \"{}\"\n", user_input).expect("Failed to Push Changes");
            println!("Project Pushed to origin");
        }
        Err(err) => println!("Error reading input: {}", err),
    }
}