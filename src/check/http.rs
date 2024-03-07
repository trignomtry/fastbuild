extern crate iron;
extern crate staticfile;
extern crate mount;
use std::path::Path;
use std::process::exit;
use iron::Iron;
use staticfile::Static;
use mount::Mount;



pub fn http() {
    let port: u16 = 9080;
    let address: &str = "127.0.0.1";
    let path: &Path = Path::new("./");

    if !path.exists() {
        println!("This directory does not exist.");
        exit(1)
    }
    if !path.is_dir() {
        println!("This is not a directory.");
        exit(1)
    }
    let mut mount: Mount = Mount::new();
    mount.mount("/", Static::new(path));

    match Iron::new(mount).http((address, port)) {
        Ok(_) => {
            println!("Http Server Available on:");
            println!("http://{}:{}", address, port);
            println!("Hit CTRL-C to stop the server")
        }
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    }
}