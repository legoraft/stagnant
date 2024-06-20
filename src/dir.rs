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
}

fn parse_frontmatter(template: String) {
    let (frontmatter, content) = split_markdown(file);
    
    let html = template.replace("{content}", &parse_markdown(&content))
        .replace("{title}", &frontmatter.title)
        .replace("{date}", &frontmatter.date)
        .replace("{description}", &frontmatter.description)
        .replace("{image}", &frontmatter.image)
        .replace("{tags}", &frontmatter.tags);
}

fn write_html() {
    
}