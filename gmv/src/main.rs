use std::{env, fs, io, process};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: {} <source> <destination>", args[0]);
        process::exit(1);
    }

    let source: &String = &args[1];
    let destination: &String = &args[2];

    fs::rename(source, destination)?;
    Ok(())
}