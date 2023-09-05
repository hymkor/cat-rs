extern crate glob;

use glob::glob;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::fmt;

fn copy_file_content_to_stdout(count: Option<i32>, filename: &str) -> Result<Option<i32>,io::Error> {
    let file = File::open(filename)?;
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

enum CatError {
    Io(io::Error),
    Glob(glob::GlobError),
    Pattern(glob::PatternError),
}

impl fmt::Display for CatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CatError::Io(e) => e.fmt(f),
            CatError::Glob(e) => e.fmt(f),
            CatError::Pattern(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for CatError {
    fn from(err: io::Error) -> CatError {
        CatError::Io(err)
    }
}

impl From<glob::GlobError> for CatError {
    fn from(err: glob::GlobError) -> CatError {
        CatError::Glob(err)
    }
}

impl From<glob::PatternError> for CatError {
    fn from(err: glob::PatternError) -> CatError {
        CatError::Pattern(err)
    }
}

fn mains(args : Vec<String>) -> Result<(),CatError> {
    let mut count : Option<i32> = None;
    for arg in &args[1..] {
        if arg == "-n" {
            count = Some(0);
            continue;
        }
        let mut glob_ok = false;
        let pattern = glob(arg)?;
        for filename in pattern {
            if let Some(filename__) = filename?.to_str() {
                count = copy_file_content_to_stdout(count,filename__)?;
                glob_ok = true;
            }
        }
        if ! glob_ok {
            count = copy_file_content_to_stdout(count,arg)?
        }
    }
    return Ok(())
}

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    if let Err(err) = mains(args) {
        eprintln!("{}",err);
        std::process::exit(1);
    }
}
