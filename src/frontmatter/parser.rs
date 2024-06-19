use yaml_rust2::{Yaml, YamlLoader};

pub fn get_yaml(frontmatter: String) -> Vec<Yaml> {
    let yaml = YamlLoader::load_from_str(&frontmatter).unwrap();
    
    yaml
}

pub fn check_value(index: &str, yaml: &Yaml) -> String {
    if yaml[index].is_badvalue() {
        return format!("{:?}", index);
    }
    
    let value: &str = yaml[index].as_str().unwrap();
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parsing() {
        
    }
}