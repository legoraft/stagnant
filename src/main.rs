use std::env;

use cli::parse_args;

mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let arguments = parse_args(args);
    
    if arguments.template.is_some() {
        
    }
}
