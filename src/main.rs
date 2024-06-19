use std::env;

use stagnant::generator;
use yaml_rust2::{YamlEmitter, YamlLoader};

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
    
    let frontmatter = "\
foo:
  - list1
  - list2
bar:
  - 1
  - 2.0".to_string();
}
