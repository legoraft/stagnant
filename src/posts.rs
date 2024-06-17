use std::{env::{current_dir, set_current_dir}, fs::{self, ReadDir}, path::PathBuf};

use crate::frontmatter::{parse, split_markdown, Frontmatter};

pub struct Post {
    filename: String,
    frontmatter: Frontmatter,
    content: String,
}

pub fn generate(posts: ReadDir, template: String) {
    let working_dir = current_dir().expect("Working directory is nonexistent.");
    let site_posts_dir = [working_dir.to_str().unwrap(), "/site/posts"].concat();
    
    write_posts(posts, template, working_dir, site_posts_dir);
}

fn write_posts(posts: ReadDir, template: String, working_dir: PathBuf, site_posts_dir: String) {
    for post in posts {
        if current_dir().unwrap() == PathBuf::from(&site_posts_dir) {
            set_current_dir(&working_dir).expect("Couldn't move to working directory.");
        }

        let path = post.expect("Couldn't get post file path!").path();
        let file = fs::read_to_string(&path).expect("Couldn't read markdown file!");
        let (frontmatter, content) = split_markdown(file);
        
        let html = template.replace("{content}", &parse_markdown(&content))
            .replace("{title}", &frontmatter.title)
            .replace("{date}", &frontmatter.date)
            .replace("{description}", &frontmatter.description);

        let filename = path.file_stem().unwrap();
        let file_path = [filename.to_str().unwrap(), ".html"].concat();
        
    }
}

fn parse_markdown(content: &str) -> String {
    let parser = pulldown_cmark::Parser::new(content);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output.replace(".md", ".html")
}