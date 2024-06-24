use std::{fs, path::Path};

use posts::Post;

mod posts;
mod frontmatter;
mod template;
mod dir;

pub fn generator() {
    let template_path = Path::new("./template");
    let site_path = Path::new("./site");
    
    template::copy_directory(template_path, site_path).expect("Couldn't copy template to site!");
    
}

pub fn old_generator() {
    let template_path = Path::new("./template");
    let site_path = Path::new("./site");
    
    template::copy_directory(template_path, site_path).expect("Couldn't copy template!");
    
    let posts = fs::read_dir("./posts").expect("Couldn't find post directory!");
    let posts_template = fs::read_to_string("./site/posts/[id].html").expect("Post template doesn't exist.");
    
    fs::remove_file("./site/posts/[id].html").expect("Couldn't delete post template!");
    
    let posts = posts::generate(posts, posts_template);
    old_write_post_list(posts);
}

fn old_write_post_list(posts: Vec<Post>) {
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
