mod check;
mod gen;
use check::http;
use crate::http::http;



fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1 {
        if args[1] == "flutter"{
            println!("Running flutter build macos...");
            std::process::Command::new("flutter")
                .args(&["build", "macos"])
                .spawn()
                .expect("failed to spawn flutter build process");
        } else if args[1] == "static" { 
            http();
        } else if args[1] == "rust" {
            std::process::Command::new("cargo")
            .args(&["run", "--debug"])
            .spawn()
            .expect("failed to spawn flutter build process");
        } else if args[1] == "gen" {
        gen::gen();
        } else {
            println!("Unknown command. Supported: run flutter");
        }
    } else {
        let _ = check::check();
    }
}
