use std::path::Path;

use crate::template;

pub fn create_site() {
    duplicate_template();
    parse_frontmatter(template);
    write_html();
}

fn duplicate_template() {
    let template_path = Path::new("./template");
    let site_path = Path::new("./site");
    
    template::copy_directory(template_path, site_path).expect("Couldn't copy template!");
    
    let posts_template = fs::read_to_string("./site/posts/[id].html").expect("Post template doesn't exist.");
    
    fs::remove_file("./site/posts/[id].html").expect("Couldn't delete post template!");
}

fn parse_frontmatter(template: String) {
    let (frontmatter, content) = split_markdown(file);
    
    let html = template.replace("{title}", &frontmatter.title)
        .replace("{date}", &frontmatter.date)
        .replace("{description}", &frontmatter.description)
        .replace("{image}", &frontmatter.image)
        .replace("{tags}", &frontmatter.tags)
        .replace("{content}", &parse_markdown(&content));
}

fn write_html() {
    
}