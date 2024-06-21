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

fn write_links() {
    let link_template = fs::read_to_string("./site/[link].html").expect("No link template found!");
    fs::remove_file("./site/[link].html").expect("Couldn't delete link template!");
    let mut link_list: String = String::new();
    
    for post in posts {
        let file_path = ["./site/posts/", &post.file_path].concat();
        
        fs::write(&file_path, &post.content).expect("Couldn't write to post!");
        
        let link = link_template
            .replace("{link}", &["./posts/", &post.file_path].concat())
            .replace("{title}", &post.frontmatter.title)
            .replace("{description}", &post.frontmatter.description)
            .replace("{date}", &post.frontmatter.date);
        
        link_list.push_str(&link);
        link_list.push('\n');
    }
    
    let index = fs::read_to_string("./site/index.html").expect("Couldn't read index!");
    
    let index_updated = index.replace("{links}", &link_list);
    fs::write("./site/index.html", &index_updated).expect("Failed to write to index.");
}