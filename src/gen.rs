use std::fs::File;
use std::io::{self, Write};

pub fn gen() {
    let mut user_input = String::new();
    println!("What type of project is this?");

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
                // Write the default build configuration for Flutter as a commented line
                writeln!(file, "\n# build = [\"flutter build\", \"flutter run\"]").expect("Failed to write to file");

                // Ask for additional build command
                println!("Enter an additional build command (leave blank to skip):");
                let mut command = String::new();
                match io::stdin().read_line(&mut command) {
                    Ok(_) => {
                        command.pop(); // Remove trailing newline
                        if !command.is_empty() {
                            writeln!(file, "build = \"{}\"", command).expect("Failed to write to file");
                        }
                    }
                    Err(err) => {
                        println!("Error reading input: {}", err);
                    }
                }

                // Write the platform-specific build commands as commented lines
                writeln!(file, "\n# Uncomment and modify the following sections to add platform-specific build commands:").expect("Failed to write to file");
                writeln!(file, "# [build.macos]").expect("Failed to write to file");
                writeln!(file, "# commands = [\"flutter build macos\"]").expect("Failed to write to file");
                writeln!(file, "\n# [build.windows]").expect("Failed to write to file");
                writeln!(file, "# commands = [\"flutter build windows\"]").expect("Failed to write to file");
            }

            println!("Project build configuration saved to .run file.");
        }
        Err(err) => println!("Error reading input: {}", err),
    }
}