use std::{fs, path::Path};

use crate::{posts::{self, Post}, template};

// Called by lib to start site generation
pub fn create_site(posts: fs::ReadDir) {
    let template = duplicate_template();

    let posts = posts::generate_posts(posts, template);
    
    write_html(posts);
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

fn write_html(posts: Vec<Post>) {
    for post in &posts {
        let post_path = ["./site/posts/", &post.file_path].concat();
        
        fs::write(post_path, &post.html).expect("Couldn't write post to file!");
    }
    
    let link_list = get_links(posts);
    let index_template = fs::read_to_string("./site/index.html").expect("Couldn't read index file!");
    
    let index = index_template.replace("{links}", &link_list);
    fs::write("./site/index.html", index).expect("Couldn't write to index!");
}

fn get_links(posts: Vec<Post>) -> String {
    // Gets the link template, which is just a simple html snippet in a separate file
    let link_template = fs::read_to_string("./site/[link].html").expect("No link template found!");
    fs::remove_file("./site/[link].html").expect("Couldn't delete link template!");
    let mut link_list: String = String::new();
    
    // Iterates over all generated posts (to prevent copying file names in the md posts dir that don't exist)
    // Gets all the data from frontmatter and replaces the variables again
    for post in posts {    
        let link = link_template
            .replace("{link}", &["./posts/", &post.file_path].concat())
            .replace("{title}", &post.frontmatter.title)
            .replace("{description}", &post.frontmatter.description)
            .replace("{date}", &post.frontmatter.date);
    
        link_list.push_str(&link);
        link_list.push('\n');
    }
    
    link_list
}