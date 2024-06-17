use std::fs::{self, ReadDir};

use crate::frontmatter::{split_markdown, Frontmatter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Post {
    pub file_path: String,
    pub frontmatter: Frontmatter,
    pub content: String,
}

pub fn generate(posts: ReadDir, template: String) -> Vec<Post> {
    let posts = write_posts(posts, template);
    
    posts
}

fn write_posts(post_list: ReadDir, template: String) -> Vec<Post> {
    let mut posts:Vec<Post> = Vec::new();
    
    for post in post_list {
        let path = post.expect("Couldn't get post file path!").path();
        let file = fs::read_to_string(&path).expect("Couldn't read markdown file!");
        let (frontmatter, content) = split_markdown(file);
        
        let html = template.replace("{content}", &parse_markdown(&content))
            .replace("{title}", &frontmatter.title)
            .replace("{date}", &frontmatter.date)
            .replace("{description}", &frontmatter.description);

        let filename = path.file_stem().unwrap();
        let file_path = [filename.to_str().unwrap(), ".html"].concat();
        
        posts.push(Post {
            file_path,
            frontmatter,
            content: html,
        })
    }
    
    posts.sort();
    posts
}

fn parse_markdown(content: &str) -> String {
    let parser = pulldown_cmark::Parser::new(content);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output.replace(".md", ".html")
}