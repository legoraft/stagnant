use parser::parse;

mod parser;

pub struct Frontmatter {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
}

pub fn parse_frontmatter(file: &str) -> Frontmatter {
    let (title, date, tags) = parse(file);
    
    Frontmatter {
        title,
        date,
        tags,
    }
}