extern crate glob;

use glob::glob;
use std::env;
use std::fs::File;
use std::io::{self,BufRead};

fn cat1<R: std::io::Read>(count: &mut Option<i32>, r: R) -> Result<(),io::Error> {
    let reader = io::BufReader::new(r);

    if let Some(mut c) = count {
        for line in reader.lines() {
            c += 1;
            println!("{}: {}", c,line?);
        }
        *count = Some(c);
    } else {
        for line in reader.lines() {
            println!("{}", line?);
        }
    }
    Ok(())
}

fn cat(args : Vec<String>) -> Result<(),Box<dyn std::error::Error>> {
    let mut count : Option<i32> = None;
    if args.len() < 2 {
        cat1(&mut count,io::stdin())?;
        return Ok(())
    }

    for arg in &args[1..] {
        if arg == "-n" {
            count = Some(0);
            continue;
        }
        if arg == "-" {
            cat1(&mut count,io::stdin())?;
            continue
        }
        let mut glob_ok = false;
        for filename in glob(arg)? {
            if let Some(filename) = filename?.to_str() {
                cat1(&mut count,File::open(filename)?)?;
                glob_ok = true;
            }
        }
        if ! glob_ok {
            cat1(&mut count,File::open(arg)?)?
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
