extern crate glob;

use std::fs::File;
use std::io::{self, BufRead};

fn cat1<R: std::io::Read>(line_count: &mut Option<i32>, r: &mut R) -> Result<(), io::Error> {
    if let Some(mut c) = line_count {
        let reader = io::BufReader::new(r);
        for line in reader.lines() {
            c += 1;
            println!("{}: {}", c, line?);
        }
        *line_count = Some(c);
    } else {
        std::io::copy(r, &mut std::io::stdout())?;
    }
    Ok(())
}

fn cat(args: std::env::Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_done = false;
    let mut line_count: Option<i32> = None;

    for arg in args.skip(1) {
        if arg == "-n" {
            line_count = Some(0);
            continue;
        }
        file_done = true;
        if arg == "-" {
            cat1(&mut line_count, &mut io::stdin())?;
            continue;
        }
        let mut glob_ok = false;
        for filename in glob::glob(&arg)? {
            if let Some(filename) = filename?.to_str() {
                cat1(&mut line_count, &mut File::open(filename)?)?;
                glob_ok = true;
            }
        }
        if !glob_ok {
            cat1(&mut line_count, &mut File::open(arg)?)?
        }
    }
    if !file_done {
        cat1(&mut line_count, &mut io::stdin())?;
    }
    return Ok(());
}

fn main() {
    if let Err(err) = cat(std::env::args()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
