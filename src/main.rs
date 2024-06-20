use std::env;

use cli::parse_args;
use stagnant::generator;

mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let arguments = parse_args();
    
    match arguments.directory {
        Some(dir) => env::set_current_dir(dir).expect("Couldn't change to specified directory."),
        None => eprintln!("No directory specified, looking for template."),
    }
    
    generator();
}
