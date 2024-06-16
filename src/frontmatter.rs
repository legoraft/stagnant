use gray_matter::{engine::YAML, Matter};

pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub date: String,
}

pub fn parse(file: String) -> Frontmatter {
    let matter = Matter::<YAML>::new();
    let frontmatter_result = matter.parse(&file);
    let frontmatter = frontmatter_result.data.as_ref().unwrap();
    
    let title = frontmatter["title"].as_string().expect("Couldn't parse title!");
    let description = frontmatter["description"].as_string().expect("Couldn't parse description!");
    let date = frontmatter["date"].as_string().expect("Couldn't parse date!");
    
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
        
    }
}