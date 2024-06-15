use std::{env::current_dir, fs::{self, ReadDir}, path::Path};

use posts::write_posts;

mod posts;

pub fn generate(posts: ReadDir, template: String) {
    if !Path::new("./site/posts").exists() {
        fs::create_dir("./site/posts/").expect("Couldn't create posts directory!")
    };
    
    let working_dir = current_dir().expect("Working directory is nonexistent.");
    let site_posts_dir = [working_dir.to_str().unwrap(), "/site/posts"].concat();
    
    write_posts(posts, template, working_dir, site_posts_dir);
}