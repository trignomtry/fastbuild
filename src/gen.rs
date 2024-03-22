use std::fs::File;
use std::io::{self, Write};

pub fn gen() {
    let mut user_input = String::new();
    println!("What type of project is this?");
    println!("Options: flutter, rust, go, static");
    // Read user input with error handling
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            // Remove trailing newline from input
            user_input.pop();
            let mut file = File::create(".run").expect("Failed to create file");
            // Write the project type
            writeln!(file, "type = \"{}\"", user_input).expect("Failed to write to file");
            // Check if user input is "flutter"
            if user_input.to_lowercase() == "flutter" {
                // Ask for additional build command
                println!("Enter an additional build command (leave blank to skip):");
                let mut command = String::new();
                match io::stdin().read_line(&mut command) {
                    Ok(_) => {
                        command.pop(); // Remove trailing newline
                        if !command.is_empty() {
                            writeln!(file, "build = [\"{}\"]", command).expect("Failed to write to file");
                        } else {
                            writeln!(file, "## Uncomment this line to run a custom build command").expect("Failed to write to file");
                            writeln!(file, "## build = []").expect("Failed to write to file");                        }
                    }
                    Err(err) => {
                        println!("Error reading input: {}", err);
                    }
                }
            }
            if user_input.to_lowercase() == "rust" {
                // Ask for additional build command
                println!("Enter an additional build command (leave blank to skip):");
                let mut command = String::new();
                match io::stdin().read_line(&mut command) {
                    Ok(_) => {
                        command.pop(); // Remove trailing newline
                        if !command.is_empty() {
                            writeln!(file, "build = [\"{}\"]", command).expect("Failed to write to file");
                            writeln!(file, "## Uncomment these lines to run custom commands, like 'debug'").expect("Failed to write to file");
                            writeln!(file, "## [commands]").expect("Failed to write to file");
                            writeln!(file, "## debug = ['echo Hello World!', 'echo Hello Developer!']").expect("Failed to write to file");
                        } else {
                            writeln!(file, "## Uncomment this line to run a custom build command").expect("Failed to write to file");
                            writeln!(file, "## build = []").expect("Failed to write to file");
                        }
                    }
                    Err(err) => {
                        println!("Error reading input: {}", err);
                    }
                }
            }
            println!("Project build configuration saved to .run file.");
        }
        Err(err) => println!("Error reading input: {}", err),
    }
}