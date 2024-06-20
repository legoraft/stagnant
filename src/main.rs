use std::env;

use stagnant::generator;
crate stagnant::cli::parse_args;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    parse_args(args);
}
