use std::env::args;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = args().collect();
    let mut path: &str = ".";
    let mut depth: Option<usize> = None;

    for i in 1..args.len() {
        if args[i] == "--depth" || args[i] == "-d" {
            if i + 1 < args.len() {
                depth = args[i + 1].parse::<usize>().ok();
            }
        } else {
            path = &args[i];
        }
    }

    let walker = match depth {
        Some(d) => WalkDir::new(path).max_depth(d).into_iter(),
        None => WalkDir::new(path).max_depth(1).into_iter(),
    };

    for entry in walker {
        match entry {
            Ok(entry) if entry.file_type().is_dir() => println!("{}", entry.path().display()),
            Err(e) => eprintln!("{:?}", e),
            _ => {}
        }
    }
}