use yaml_rust2::{Yaml, YamlLoader};

pub fn get_yaml(frontmatter: String) -> Vec<Yaml> {
    // Gets all yaml from a frontmatter string
    let yaml = YamlLoader::load_from_str(&frontmatter).unwrap();
    
    yaml
}

pub fn check_value(index: &str, yaml: &Yaml) -> String {
    // Checks if yaml exists, otherwise return field name
    if yaml[index].is_badvalue() {
        return format!("{}", index);
    }
    
    let value: &str = yaml[index].as_str().unwrap();
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_value() {
        let frontmatter = "\
title: \"Hello, world!\"
date: \"2024-06-19\"";
        
        let yaml = YamlLoader::load_from_str(frontmatter).unwrap();
        
        assert_eq!("Hello, world!", check_value("title", &yaml[0]));
    }
}