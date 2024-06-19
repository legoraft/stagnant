use gray_matter::{engine::YAML, Matter, ParsedEntity};
use parser::{check_value, get_yaml};
use yaml_rust2::Yaml;

mod parser;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    pub description: String,
    pub image: String,
    pub tags: String,
}

pub fn split_markdown(file: String) -> (Frontmatter, String) {
    let matter = Matter::<YAML>::new();
    let frontmatter_result = matter.parse(&file);
    
    let yaml = get_yaml(frontmatter_result.matter);
    let frontmatter = parse(yaml);
    let content = frontmatter_result.content;
    
    (frontmatter, content)
}

 fn parse(yaml: Vec<Yaml>) -> Frontmatter {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frontmatter() {
        let file: String = "\
---
title: Test post
date: 2023-06-16
description: A fake test post to have as a test case.
---

This is where te body of the post would go normally.".to_string();
        
        let matter = Matter::<YAML>::new();
        let frontmatter_result = matter.parse(&file); 
        
        let frontmatter = Frontmatter {
            title: "Test post".to_string(),
            description: "A fake test post to have as a test case.".to_string(),
            date: "2023-06-16".to_string(),
        };
        
        assert_eq!(frontmatter, parse(&frontmatter_result));
    }
}