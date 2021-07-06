use std::path::PathBuf;
use structopt::StructOpt;

pub use utils::type_of;

mod utils;



#[derive(Debug, StructOpt)]
#[structopt(name="grrs example", about="An example for command line app.")]
pub struct Cli {
    /// the pattern to look for
    #[structopt(short, long)]
    pub pattern: String,

    /// the path to the file to read
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,

    #[structopt(short, long, parse(from_os_str), default_value="./")]
    pub output: PathBuf,
}