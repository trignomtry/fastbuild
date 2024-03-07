use toml::Value;
use std::{error::Error, ffi::OsStr, path::Path};
use::std::fs;
pub(crate) mod http;
use http::http;

const file_path: &str = ".run";

pub fn check() -> Result<(), Box<dyn Error>> {
    if !std::path::Path::new(file_path).exists() {
        println!("File '{}' not found.", file_path);
        return Ok(());
    }
    let contents = fs::read_to_string(file_path)?;
    let toml_value: Value = toml::from_str(&contents)?;
    if toml_value.is_table() && !toml_value.as_table().unwrap().is_empty() {
        let _ = build_command();
        for (key, value) in toml_value.as_table().unwrap().iter() {
            let val: &str = value.as_str().unwrap();
            if key == "type" {
                if val == "flutter" {
                    println!("Running flutter build macos...");
                    std::process::Command::new("flutter")
                        .args(&["build", "macos"])
                        .spawn()
                        .expect("failed to spawn flutter build process");
                } else if val == "static" {
                    http();
                } else if val == "wow" {
                    println!("That's not a real type, but at least my project works.")
                }
            } if key == "file" {
                println!("found file");
                let wow = ext(val);
                if wow == Some("go") { 
                    print!("found another file");
                    std::process::Command::new("go")
                    .args(&["run", val])
                    .spawn()
                    .expect("failed to run go project");
                }
            }
        }
    } else {
        if toml_value.is_table() {
            println!("The table in the file '.run' is empty.");
        } else {
            println!("The file '.run' does not contain a table.");
        }
    }

    Ok(())
}





fn ext(filename: &str) -> Option<&str> {
    Path::new(filename)
      .extension()
      .and_then(OsStr::to_str)
  }


pub fn build_command() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let toml_value: Value = toml::from_str(&contents)?;

    if let Some(build) = toml_value.get("build") {
        if build.is_array() {
            for command in build.as_array().unwrap() {
                run_command(command.as_str().unwrap_or_default());
            }
        }
    }

    if let Some(build_table) = toml_value.get("build") {
        if build_table.is_table() {
            for (target, commands) in build_table.as_table().unwrap() {
                if let Some(commands) = commands.as_array() {
                    for command in commands {
                        run_command(&format!("{} {}", target, command.as_str().unwrap_or_default()));
                    }
                }
            }
        }
    }

    Ok(())
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