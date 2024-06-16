use gray_matter::{engine::YAML, Matter};

#[derive(Debug, PartialEq, Eq)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
}

pub fn parse(file: String) -> Frontmatter {
    let matter = Matter::<YAML>::new();
    let frontmatter_result = matter.parse(&file);
    let frontmatter = frontmatter_result.data.as_ref().unwrap();
    
    let title: Option<String> = Some(frontmatter["title"].as_string().unwrap_or_default());
    let description: Option<String> = Some(frontmatter["description"].as_string().unwrap_or_default());
    let date: Option<String> = Some(frontmatter["date"].as_string().unwrap_or_default());
    
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