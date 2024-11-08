use std::env;
use std::fs::{remove_dir_all, remove_dir, remove_file};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target: &str = "file";
    let mut mode: &str = "normal";

    if args.len() < 2 {
        eprintln!("Usage: {} [-r] [-f] <path>", args[0]);
        return;
    }

    let path: &String;
    let flags: &String = &args[1];

    if flags.starts_with("-") {
        path = &args[2];
        for flag in flags.chars() {
            match flag {
                'r' => target = "folder",
                'f' => mode = "force",
                _ => {}
            }
        }
    } else {
        path = &args[1];
    }

    if target == "file" {
        match remove_file(path) {
            Ok(_) => println!("removed file: {}", path),
            Err(e) => println!("{}", e)
        }
    } else if target == "folder" {
        if mode == "force" {
            for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() {
                    match remove_file(entry.path()) {
                        Ok(_) => println!("removed file: {}", entry.path().display()),
                        Err(e) => println!("error removing file: {}\nreason: {}", entry.path().display(), e)
                    }
                }
            }

            match remove_dir_all(path) {
                Ok(_) => println!("removed folder and its contents: {}", path),
                Err(e) => println!("error: failed to remove: {}\nreason: {}", path, e)
            }
        } else {
            match remove_dir(path) {
                Ok(_) => println!("removed folder: {}", path),
                Err(e) => println!("error: failed to remove: {}\nreason: {}\nhint: try with -f flag", path, e)
            }
        }
    }
}