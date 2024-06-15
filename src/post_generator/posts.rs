use std::{env::{current_dir, set_current_dir}, fs::{self, ReadDir}, path::PathBuf};

use pulldown_cmark::Options;

use crate::frontmatter::parse_frontmatter;

pub fn write_posts(posts: ReadDir, template: String, working_dir: PathBuf, site_posts_dir: String) {
    for post in posts {
        if current_dir().unwrap() == PathBuf::from(&site_posts_dir) {
            set_current_dir(&working_dir).expect("Couldn't move to working directory.");
        }

        let path = post.expect("Couldn't get post file path!").path();
        let file = fs::read_to_string(&path).expect("Couldn't read markdown file!");
        let content = parse_markdown(&file);
        let data = parse_frontmatter(&file);
        let html = template.replace("{content}", &content);

        let filename = path.file_stem().unwrap();
        let output_file = [filename.to_str().unwrap(), ".html"].concat();
        
        set_current_dir(&site_posts_dir).expect("Couldn't move into site posts dir!");
        fs::write(output_file, html).expect("Couldn't write post file!");
    }
}

fn parse_markdown(file: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    
    let parser = pulldown_cmark::Parser::new_ext(file, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    
    html_output.replace(".md", ".html")
}