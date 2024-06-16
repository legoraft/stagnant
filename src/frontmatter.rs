use gray_matter::{engine::YAML, Matter};

#[derive(Debug, PartialEq, Eq)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub date: String,
}

pub fn parse(file: String) -> Frontmatter {
    let matter = Matter::<YAML>::new();
    let frontmatter_result = matter.parse(&file);
    
    let title: String = frontmatter_result.data
        .as_ref()
        .unwrap()["title"]
        .as_string().expect("Couldn't parse title");
    
    let description: String = frontmatter_result.data
        .as_ref()
        .unwrap()["description"]
        .as_string().expect("Couldn't parse description");
    
    let date: String = frontmatter_result.data
        .as_ref()
        .unwrap()["date"]
        .as_string().expect("Couldn't parse date");
    
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
            title: "Test post".to_string(),
            description: "A fake test post to have as a test case.".to_string(),
            date: "2023-06-16".to_string(),
        };
        
        assert_eq!(frontmatter, parse(file));
    }
}