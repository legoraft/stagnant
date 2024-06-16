use std::collections::hash_map::Values;

use gray_matter::{engine::YAML, Matter, ParsedEntity};

#[derive(Debug, PartialEq, Eq)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
}

pub fn parse(file: String) -> Frontmatter {
    let matter = Matter::<YAML>::new();
    let frontmatter_result = matter.parse(&file);
    
    let title: Option<String> = Some(frontmatter_result.data
        .as_ref()
        .unwrap()["title"]
        .as_string().expect("Couldn't parse title"));
    
    let description: Option<String> = Some(frontmatter_result.data
        .as_ref()
        .unwrap()["description"]
        .as_string().expect("Couldn't parse description"));
    
    let date: Option<String> = Some(frontmatter_result.data
        .as_ref()
        .unwrap()["date"]
        .as_string().expect("Couldn't parse date"));
    
    Frontmatter {
        title,
        description,
        date
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
        
        let frontmatter = Frontmatter {
            title: Some("Test post".to_string()),
            description: Some("A fake test post to have as a test case.".to_string()),
            date: Some("2023-06-16".to_string()),
        };
        
        assert_eq!(frontmatter, parse(file));
    }
}