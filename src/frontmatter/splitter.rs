pub fn split_post(file: String) -> (String, String) {
    let mut matter: String = String::new();
    let mut content: String = String::new();
    
    let lines: Vec<&str> = file.lines().collect();
    let mut in_matter = false;
    
    // Checks if first line is a frontmatter delimiter
    if &lines[0] == &"---" {
        in_matter = true;
    }
    
    // Until the next frontmatter delimiter is found, copy lines to matter
    // Else, copy lines to content
    for line in &lines[1..] {
        if in_matter && line == &"---" {
            in_matter = false;
            continue;
        } else if in_matter {
            matter.push_str(line);
            matter.push('\n');
        } else {
            content.push_str(line);
            content.push('\n')
        }
    }
    
    matter = matter.trim().to_string();
    content = content.trim().to_string();
    
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
        
        let (matter, content) = ("title: \"Hello, world!\"\ndate: \"2024-06-21\"".to_string(), "This is some content to test if we also get this.\n\nI also want an extra line to check if it works completely.".to_string());
        
        assert_eq!((matter, content), split_post(file));
    }
}