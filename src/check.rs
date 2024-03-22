use toml::Value;
use std::{error::Error, ffi::OsStr, path::Path};
use::std::fs;
pub(crate) mod http;
use http::http;

const FILE_PATH: &str = ".run";

pub fn check() -> Result<(), Box<dyn Error>> {
    if !std::path::Path::new(FILE_PATH).exists() {
        println!("File '{}' not found.", FILE_PATH);
        return Ok(());
    }
    let contents = fs::read_to_string(FILE_PATH)?;
    let toml_value: Value = toml::from_str(&contents)?;
    if toml_value.is_table() && !toml_value.as_table().unwrap().is_empty() {
        let _ = build_command();
        for (key, value) in toml_value.as_table().unwrap().iter() {
            let val: &str = value.as_str().unwrap_or_default();
            if key == "type" {
                if val == "flutter" {
                    println!("Running flutter build macos...");
                    std::process::Command::new("flutter")
                        .args(&["build", "macos"])
                        .spawn()
                        .expect("failed to spawn flutter build process");
                } else if val == "static" {
                    http();
                } else if val == "rust" {              
                    if build_command() == false {
                        std::process::Command::new("cargo")
                        .args(&["run"])
                        .spawn()
                        .expect("failed to spawn cargo run process");
                    }
                } else if val == "wow" {
                    println!("That's not a real type, but at least my project works.")
                } else {
                    println!("{} is not a valid type.", val);
                }
            } if key == "file" {
                let wow = ext(val);
                if wow == Some("go") { 
                    std::process::Command::new("go")
                    .args(&["run", val])
                    .spawn()
                    .expect("failed to run go project");
                }
            }
            std::process::exit(0)
        }
        std::process::exit(0)
    } else {
        if toml_value.is_table() {
            println!("The table in the file '.run' is empty.");
        } else {
            println!("The file '.run' does not contain a table.");
        }
    }

    //Ok(());
    std::process::exit(0)
}

fn ext(filename: &str) -> Option<&str> {
    Path::new(filename)
      .extension()
      .and_then(OsStr::to_str)
}

pub fn build_command() -> bool {
    let contents = fs::read_to_string(FILE_PATH);
    let toml_value: Value = toml::from_str(&contents.unwrap()).unwrap();

    if let Some(build) = toml_value.get("build") {
        if build.is_array() {
            for command in build.as_array().unwrap() {
                run_command(command.as_str().unwrap_or_default());
                return true
            }
        } 
    } else {
         return false
    }
return false
}

fn run_command(command: &str) {
    let mut parts = command.split_whitespace();
    let cmd = parts.next().unwrap();
    let args: Vec<_> = parts.collect();

    std::process::Command::new(cmd)
        .args(&args)
        .spawn()
        .expect("failed to spawn process");
}









use std::collections::HashMap;
use toml;

pub fn command(command: &str) -> bool {
    let contents = fs::read_to_string(FILE_PATH);
    let parsed: Value = toml::from_str(&contents.unwrap()).unwrap();
    let commands = parsed["commands"].as_table().unwrap();

    let mut result = HashMap::new();
    if commands.len() == 0 {
        println!("No commands found in .run file.");
        return false
    } else {
        for (key, value) in commands {
            let values = value.as_array().unwrap();
            let command_strings: Vec<String> = values
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect();
            result.insert(key.to_string(), command_strings.clone());
            if command == key {
                for command in command_strings {
                    let mut parts = command.split_whitespace();
                    let cmd = parts.next().unwrap();
                    let args: Vec<_> = parts.collect();
                    std::process::Command::new(cmd)
                        .args(&args)
                        .spawn()
                        .expect("failed to spawn process");
                }
                break
            } else {
                println!("Command '{}' not found in .run file.", command);
            }
        }
    }

    false
}