use std::env;
use std::fs::File;
use std::io::{self,Read};

fn copy_file_content_to_stdout(filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    print!("{}", buffer);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    if let Err(err) = copy_file_content_to_stdout(filename) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

