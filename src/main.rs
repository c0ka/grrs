use std::io::{BufReader, BufRead};
use std::fs;
use std::error;
use std::process;
use structopt::StructOpt;

// this is how we use lib.rs
use grrs::{type_of, Cli};

/// example:
/// grrs ./ --pattern test1

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    let args = Cli::from_args();
    if let Err(err) = try_main(args) {
        eprintln!("{}", err);
        process::exit(2);
    }
    
}

fn try_main(args: Cli) -> Result<()> {
    let content = fs::File::open(&args.path)?;
    let reader = BufReader::new(content);

    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    println!("Hello, world! for {:?}", args);
    println!("type of args is: {:?}", type_of(args));
    Ok(())
}
