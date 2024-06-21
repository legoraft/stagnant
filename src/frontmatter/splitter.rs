pub fn split_post(file: String) -> (String, String) {
    let mut matter: String = String::new();
    let mut content: String = String::new();
    
    let lines: Vec<&str> = file.lines().collect();
    let mut in_matter = false;
    
    if &lines[0] == &"---" {
        in_matter = true;
    }
    
    for line in &lines[1..] {
        if in_matter && line == &"---" {
            in_matter = false;
            continue;
        } else if in_matter {
            matter.push_str(line);
        } else {
            content.push_str(line);
        }
    }
    
    (matter, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split() {
        let file = "\
---
title: \"Hello, world!\"
date: \"2024-06-21\"
---

This is some content to test if we also get this.

I also want an extra line to check if it works completely.".to_string();
        
        let (matter, content) = ("title: \"Hello, world!\"\ndate: \"2024-06-21\"".to_string(), "\n\nThis is some content to test if we also get this.\n\nI also want an extra line to check if it works completely.".to_string());
        
        assert_eq!((matter, content), split_post(file));
    }
}