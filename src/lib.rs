use std::{fs, path::Path};

mod post_generator;

pub fn generator() {
    let posts = fs::read_dir("./posts").expect("Couldn't find post directory!");
    let posts_template = fs::read_to_string("./template/posts/[id].html").expect("Post template doesn't exist.");

    if !Path::new("./site").exists() {
        fs::create_dir("./site").expect("Couldn't create site directory!");
    }

    post_generator::generate(posts, posts_template);
}