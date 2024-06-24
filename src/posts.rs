use std::fs::{self, ReadDir};

use crate::frontmatter::{self, Frontmatter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Post {
    pub file_path: String,
    pub frontmatter: Frontmatter,
    pub html: String,
}

pub fn generate_posts(posts: ReadDir, template: String) -> Vec<Post> {
    let posts = get_posts(posts, template);
    
    posts
}

fn get_posts(post_list: ReadDir, template: String) -> Vec<Post> {
    // Generates a vector of a Post struct, which contains the filename, frontmatter fields and the html content
    let mut posts:Vec<Post> = Vec::new();
    
    for post in post_list {
        // Reads all markdown from a post
        let path = post.expect("Couldn't get post file path!").path();
        let file = fs::read_to_string(&path).expect("Couldn't read markdown file!");
        
        // Parses all frontmatter and replaces tags in html template
        let (matter, content) = frontmatter::parse(file);
        let html = replace_tags(&content, &template, &matter);

        // Gets full html filename
        let filename = path.file_stem().unwrap();
        let file_path = [filename.to_str().unwrap(), ".html"].concat();
        
        posts.push(Post {
            file_path,
            frontmatter: matter,
            html,
        })
    }
    
    // Sorts all the posts by date, which is formatted by YYYY-MM-DD
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
    posts
}

fn replace_tags(content: &str, template: &str, matter: &Frontmatter) -> String {
    // Replaces all custom tags which are used in html templates
    let html = template.replace("{title}", &matter.title)
        .replace("{date}", &matter.date)
        .replace("{description}", &matter.description)
        .replace("{image}", &matter.image)
        .replace("{tags}", &matter.tags)
        .replace("{content}", &parse_markdown(&content));
    
    html
}

fn parse_markdown(content: &str) -> String {
    let parser = pulldown_cmark::Parser::new(content);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Replaces internal md links to html files
    html_output.replace(".md", ".html")
}