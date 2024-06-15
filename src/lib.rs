use std::{env::current_dir, fs::{self, ReadDir}, path::Path};

mod post_generator;
mod frontmatter;

pub fn generator() {
    let posts = fs::read_dir("./posts").expect("Couldn't find post directory!");
    let posts_template = fs::read_to_string("./template/posts/[id].html").expect("Post template doesn't exist.");
    let index_template = fs::read_to_string("./template/index.html").expect("Index template doesn't exist.");

    if !Path::new("./site").exists() {
        fs::create_dir("./site").expect("Couldn't create site directory!");
    }
    
    post_generator::generate(posts, posts_template);
    write_index(index_template);
}

fn write_index(template: String) {
    let posts = fs::read_dir("./site/posts").expect("Posts haven't compiled!");
    let mut post_list: String = String::new();
    
    for post in posts {
        let path = post.unwrap().path();
        let filename = path.file_stem().unwrap().to_str().unwrap();
        let link = ["<li><a href=\"", filename, ".html\">", filename, "</a></li>\n"].concat();
        post_list.push_str(link.as_str());
    }
    
    let index = template.replace("{links}", &post_list);
    fs::write("./site/index.html", index).expect("Couldn't write index file!");
}
