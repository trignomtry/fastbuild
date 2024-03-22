mod check;
mod gen;
use check::http;
use crate::http::http;
mod new;



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
            .args(&["run"])
            .spawn()
            .expect("failed to spawn flutter build process");
        } else if args[1] == "gen" {
        gen::gen();
        } else if args[1] == "new" {
            let newtype = args[2].as_str();
            let mut var_name = args[3].clone();
            var_name = String::from(var_name);
            let _ = new::new(newtype, &var_name);
        } 
        else {
            check::command(args[1].as_str());
        }
    } else {
        let _ = check::check();
    }
}