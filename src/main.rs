use std::env;

use stagnant::generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        generator();
    } else if args.len() == 2 {
        env::set_current_dir(&args[1]).expect("Couldn't move into specified directory!");
        generator();
    } else {
        eprintln!("Only add a site directory as argument.");
    }

}
