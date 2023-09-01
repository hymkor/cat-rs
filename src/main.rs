use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn copy_file_content_to_stdout(count: Option<i32>, filename: &str) -> Result<Option<i32>,io::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => { return Err(e) }
    };
    let reader = BufReader::new(file);

    if let Some(mut c) = count {
        for line in reader.lines() {
            c += 1;
            println!("{}: {}", c,line?);
        }
        Ok(Some(c))
    } else {
        for line in reader.lines() {
            println!("{}", line?);
        }
        Ok(None)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let mut count : Option<i32> = None;
    for filename in &args[1..] {
        if filename == "-n" {
            count = Some(0)
        } else {
            match copy_file_content_to_stdout(count,filename){
                Ok(c) => count = c,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                }
            }
        }
    }
}

