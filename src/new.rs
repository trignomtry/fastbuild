use std::fs;
use std::io::Write;
use std::path::Path;

pub fn new(newtype: &str, directory_name: &str) -> std::io::Result<()> {
    // Create the directory
    fs::create_dir(directory_name)?;

    // Create the .run file
    let run_file_path = Path::new(directory_name).join(".run");
    let mut run_file = fs::File::create(run_file_path)?;
    writeln!(run_file, "type = \"{}\"", newtype)?;

    if newtype == "static" {
        let html_file_path = Path::new(directory_name).join("index.html");
        let mut html_file = fs::File::create(html_file_path)?;
        writeln!(
            html_file,
            "<!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>{directory_name}</title>
    </head>
    <body>
    </body>
    </html>"
        )?;
    } else if newtype == "flutter" {
        println!("Making new flutter project...");
        std::process::Command::new("flutter")
            .args(&["new", directory_name])
            .spawn()
            .expect("failed to spawn flutter new process");
    } else if newtype == "rust" {
        println!("Making new rust project...");
        std::process::Command::new("cargo")
            .args(&["init", directory_name])
            .spawn()
            .expect("failed to spawn cargo new process");
    } 

    Ok(())
}