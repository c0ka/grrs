use std::io::{BufReader, BufRead};
use std::fs;
use std::error;

// this is how we use lib.rs
use grrs::{type_of, Cli};
use structopt::StructOpt;

/// example:
/// grrs ./ --pattern test1


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let args = Cli::from_args();

    let content = fs::File::open(&args.path)?;
        // .expect("could not read file");
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
