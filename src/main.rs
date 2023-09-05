extern crate glob;

use glob::glob;
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

fn mains(args : Vec<String>) -> Option<String> {
    if args.len() < 2 {
        return Some(format!("Usage: {} <filename>", args[0]))
    }

    let mut count : Option<i32> = None;
    for arg in &args[1..] {
        if arg == "-n" {
            count = Some(0);
            continue;
        }
        match glob(arg) {
            Ok(arg) => {
                for filename in arg {
                    match filename {
                        Ok(filename_) => {
                            if let Some(filename__) = filename_.to_str() {
                                match copy_file_content_to_stdout(count,filename__){
                                    Ok(c) => count = c,
                                    Err(err) => {
                                        return Some(format!("Error: {}", err))
                                    }
                                }
                            }
                        },
                        Err(err) => {
                            return Some(format!("Error: {}", err))
                        }
                    }
                }
            },
            Err(_) => {
                match copy_file_content_to_stdout(count,arg){
                    Ok(c) => count = c,
                    Err(err) => {
                        return Some(format!("Error: {}", err));
                    }
                }
            }
        }
    }
    return None
}

fn main(){
    if let Some(message) = mains(env::args().collect()) {
        eprintln!("{}",message);
        std::process::exit(1);
    }
}
