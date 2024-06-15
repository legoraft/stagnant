use std::env;

use stagnant::generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    env::set_current_dir(&args[1]).expect("Couldn't move into specified directory!");
    generator();
}
