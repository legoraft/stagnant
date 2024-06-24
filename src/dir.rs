use std::{fs, path::Path};

use crate::{frontmatter, template};

// Called by lib to start site generation
pub fn create_site() {
    let template = duplicate_template();
    parse_frontmatter(template);
    write_html();
}

fn duplicate_template() -> String {
    // Copies template to the site dir, returns a template for the posts (html)
    let template_path = Path::new("./template");
    let site_path = Path::new("./site");
    
    template::copy_directory(template_path, site_path).expect("Couldn't copy template!");
    
    let posts_template = fs::read_to_string("./site/posts/[id].html").expect("Post template doesn't exist.");
    fs::remove_file("./site/posts/[id].html").expect("Couldn't delete post template!");
    
    posts_template
}

fn parse_frontmatter(template: String, file: String) {
    // Gets all frontmatter and content from a markdown file
    // Removes the --- from frontmatter and parses the yaml to a Frontmatter struct
    let (matter, content) = frontmatter::parse(file);
    
    // Replaces all possible variables in html to the correct values
    // Stagnant uses a limited amount of variables, so this is the effective way
    let html = template.replace("{title}", &matter.title)
        .replace("{date}", &matter.date)
        .replace("{description}", &matter.description)
        .replace("{image}", &matter.image)
        .replace("{tags}", &matter.tags)
        .replace("{content}", &parse_markdown(&content));
}

fn write_html() {
    
}

fn write_links() {
    // Gets the link template, which is just a simple html snippet in a separate file
    let link_template = fs::read_to_string("./site/[link].html").expect("No link template found!");
    fs::remove_file("./site/[link].html").expect("Couldn't delete link template!");
    let mut link_list: String = String::new();
    
    // Iterates over all generated posts (to prevent copying file names in the md posts dir that don't exist)
    // Gets all the data from frontmatter and replaces the variables again
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
    
    // The full list of link snippets is written to the specified link list file
    let index = fs::read_to_string("./site/index.html").expect("Couldn't read index!");
    
    let index_updated = index.replace("{links}", &link_list);
    fs::write("./site/index.html", &index_updated).expect("Failed to write to index.");
}