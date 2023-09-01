use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn copy_file_content_to_stdout(filename: &str) -> io::Result<()> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => { return Err(e) }
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    for filename in &args[1..] {
        if let Err(err) = copy_file_content_to_stdout(filename) {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

