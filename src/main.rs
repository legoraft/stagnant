use std::env;

use clap::Parser;
use cli::Args;
use stagnant::old_generator;

mod cli;

fn main() {
    let arguments = Args::parse();
    
    match arguments.directory {
        Some(dir) => env::set_current_dir(dir).expect("Couldn't change to specified directory."),
        None => eprintln!("No directory specified, looking for template."),
    }
    
    old_generator();
}
