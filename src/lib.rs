use std::{env::{current_dir, set_current_dir}, fs, path::Path};

mod post_generator;
mod frontmatter;
mod template;

pub fn generator() {
    let template_path = Path::new("./template");
    let site_path = Path::new("./site");
    
    template::copy_directory(template_path, site_path).expect("Couldn't copy template!");
    
    let posts = fs::read_dir("./posts").expect("Couldn't find post directory!");
    let posts_template = fs::read_to_string("./site/posts/[id].html").expect("Post template doesn't exist.");
    let index_template = fs::read_to_string("./site/index.html").expect("Index template doesn't exist.");
    
    post_generator::generate(posts, posts_template);
    write_index(index_template);
}

fn write_index(template: String) {
    set_current_dir(current_dir().unwrap().parent().unwrap()).expect("Couldn't move to working directory.");
    let posts = fs::read_dir("./posts").expect("Posts haven't compiled!");
    let mut post_list: String = String::new();
    
    for post in posts {
        let path = post.unwrap().path();
        let filename = path.file_stem().unwrap().to_str().unwrap();
        let link = ["<li><a href=\"./posts/", filename, ".html\">", filename, "</a></li>\n"].concat();
        post_list.push_str(link.as_str());
    }
    
    let index = template.replace("{links}", &post_list);
    fs::write("./index.html", index).expect("Couldn't write index file!");
}
