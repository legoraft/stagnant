use std::io::Error;

use yaml_rust2::{Yaml, YamlLoader};

use super::*;

fn parse(frontmatter: String) -> Vec<Yaml> {
    let yaml = YamlLoader::load_from_str(&frontmatter).unwrap();
    
    yaml
}

fn check_value(index: &str, yaml: Yaml) -> String {
    if yaml[index].is_badvalue() {
        return format!("{:?}", index);
    }
    
    let value: &str = yaml[index].as_str().unwrap();
    value.to_string()
}

/*
Design:

Get frontmatter from gray_matter with parse.matter()

Parse frontmatter from yaml. Check if value exists, if exists add value in struct. Otherwise add {tag} to struct.

iterate over all possible values with for loop. Have variable; let values: ["title", "date", "description", "image", "tags"];

Usage:
let title = check_value("title"); (Would result in either String or {title} tag)

Implementation:
fn check_value(value: &str) -> String {
    if value.isbad {
        return format("{value}");
    }
    return yaml[value].tostring;
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parsing() {
        
    }
}