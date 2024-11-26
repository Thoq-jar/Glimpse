use colored::Colorize;
use std::io::Result;
use std::env::args;
use walkdir::{DirEntry, WalkDir};

fn is_item(item: DirEntry) -> bool {
    if item.file_type().is_dir() || item.file_type().is_file() {
        true
    } else {
        false
    }
}

fn main() -> Result<()> {
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
            Ok(entry) if is_item(entry.clone()) => {
                let mut output = if entry.file_type().is_file() {
                    format!("{}{}", "📄", entry.path().display())
                } else {
                    format!("{}{}", "📁", entry.path().display())
                };

                output = output.replace("./", "").replace(".\\", "");

                if output == "📁." {
                    continue;
                }

                if entry.file_type().is_dir() && output.starts_with('.') {
                    output = format!("📁{}", output.trim_start_matches('.'));
                }

                if output.starts_with('.') && output != "📁." {
                    output = output.trim_start_matches('.').to_string();
                }

                if entry.file_type().is_file() {
                    println!("{}", output.green());
                } else {
                    println!("{}", output.blue());
                }
            }
            Err(e) => eprintln!("{}", e),
            _ => {}
        }
    }

    Ok(())
}
