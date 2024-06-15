use parser::parse;

mod parser;

pub struct Frontmatter {
    title: String,
    date: String,
    tags: Vec<String>,
}

pub fn parse_frontmatter(file: &str) {
    parse(file);
}