use structopt::StructOpt;

use renamer::{args::Options, run};
use std::process;

fn main() {
    let options = Options::from_args();

    if let Err(error) = run(options) {
        eprintln!("{} Exiting.", error);
        process::exit(1);
    }
}
