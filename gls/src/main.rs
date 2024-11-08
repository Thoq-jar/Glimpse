use std::env::args;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = args().collect();
    let mut path: &str = ".";

    if args.len() > 1 {
        path = &args[1];
    }

    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}