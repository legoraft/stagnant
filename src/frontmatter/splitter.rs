

pub fn split_post(file: String) -> (String, String) {
    let yaml = get_yaml(frontmatter_result.matter);
    let frontmatter = parse(yaml);
    let content = frontmatter_result.content;
    
    (frontmatter, content)
}