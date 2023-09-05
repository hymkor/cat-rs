extern crate glob;

use glob::glob;
use std::env;
use std::fs::File;
use std::io::{self,BufRead};
use std::fmt;

fn cat1<R: std::io::Read>(count: Option<i32>, r: R) -> Result<Option<i32>,io::Error> {
    let reader = io::BufReader::new(r);

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

fn cat(args : Vec<String>) -> Result<(),CatError> {
    if args.len() < 2 {
        cat1(None,io::stdin())?;
        return Ok(())
    }

    let mut count : Option<i32> = None;
    for arg in &args[1..] {
        if arg == "-n" {
            count = Some(0);
            continue;
        }
        if arg == "-" {
            count = cat1(count,io::stdin())?;
            continue
        }
        let mut glob_ok = false;
        for filename in glob(arg)? {
            if let Some(filename) = filename?.to_str() {
                count = cat1(count,File::open(filename)?)?;
                glob_ok = true;
            }
        }
        if ! glob_ok {
            count = cat1(count,File::open(arg)?)?
        }
    }
    return Ok(())
}

fn main(){
    if let Err(err) = cat(env::args().collect()) {
        eprintln!("{}",err);
        std::process::exit(1);
    }
}
