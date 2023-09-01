use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn copy_file_content_to_stdout(mut count: i32, filename: &str) -> Result<i32,io::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => { return Err(e) }
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        count += 1;
        println!("{}: {}", count,line?);
    }
    Ok(count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let mut count = 0;
    for filename in &args[1..] {
        match copy_file_content_to_stdout(count,filename){
            Ok(c) => count = c,
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }
}

