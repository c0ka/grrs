use std::path::PathBuf;
use structopt::StructOpt;

// this is how we use lib.rs
use grrs::type_of;

mod single_linked;
mod persis_linked;

/// example:
/// grrs ./ --pattern test1
#[derive(Debug, StructOpt)]
#[structopt(name="grrs example", about="An example for command line app.")]
struct Cli {
    /// the pattern to look for
    #[structopt(short, long)]
    pattern: String,

    /// the path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    #[structopt(short, long, parse(from_os_str), default_value="./")]
    output: PathBuf,
}

fn main() {
    let args = Cli::from_args();

    println!("Hello, world! for {:?}", args);
    println!("type of args is: {:?}", type_of(args));
}
