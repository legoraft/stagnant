use splitter::split_post;
use yaml_parser::{check_value, get_yaml};
use yaml_rust2::Yaml;

mod yaml_parser;
mod splitter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    pub description: String,
    pub image: String,
    pub tags: String,
}

impl Frontmatter {
    // Checks if frontmatter values exist and writes them to Frontmatter struct
    fn get(yaml: Vec<Yaml>) -> Self {
        let yaml = &yaml[0];
    
        let title: String = check_value("title", &yaml);
        let description: String = check_value("description", &yaml);
        let date: String = check_value("date", &yaml);
        let image: String = check_value("image", &yaml);
        let tags: String = check_value("tags", &yaml);
    
        Frontmatter {
            title,
            date,
            description,
            image,
            tags
        }
    }
}

pub fn parse(file: String) -> (Frontmatter, String) {
    // Splits frontmatter and content and creates Frontmatter struct from it
    let (matter, content) = split_post(file);
    
    let yaml = get_yaml(matter);
    let matter = Frontmatter::get(yaml);
    
    (matter, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frontmatter() {
        let file: String = "\
---
title: \"Test post\"
date: \"2023-06-16\"
description: \"A fake test post to have as a test case.\"
image: \"images/image.png\"
tags: \"test, hello, world\"
---

This is where te body of the post would go normally.".to_string();
        
        let matter = Matter::<YAML>::new();
        let frontmatter_result = matter.parse(&file);
        let yaml = get_yaml(frontmatter_result.matter);
        
        let frontmatter = Frontmatter {
            title: "Test post".to_string(),
            date: "2023-06-16".to_string(),
            description: "A fake test post to have as a test case.".to_string(),
            image: "images/image.png".to_string(),
            tags: "test, hello, world".to_string(),
        };
        
        assert_eq!(frontmatter, Frontmatter::get(yaml));
    }
}