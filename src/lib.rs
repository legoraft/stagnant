use std::fs;

use dir::create_site;

mod posts;
mod frontmatter;
mod template;
mod dir;

pub fn generator() {
    let posts = fs::read_dir("./posts").expect("Couldn't find post directory!");
    
    create_site(posts)
}