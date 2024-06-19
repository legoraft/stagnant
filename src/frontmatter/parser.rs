use std::io::Error;

use yaml_rust2::{Yaml, YamlLoader};

fn parse(frontmatter: String) -> Vec<Yaml> {
    let yaml = YamlLoader::load_from_str(&frontmatter).unwrap();
    
    yaml
}

fn check_value(index: &str, yaml: Vec<Yaml>) -> Result<String, Error> {
    let yaml = yaml[0];
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parsing() {
        
    }
}