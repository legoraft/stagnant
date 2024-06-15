use regex::Regex;

pub fn parse(file: &str)  -> (String, String, String) {
    let frontmatter_re = Regex::new(r"(?s)---(?<f>.*?)---(.*)").unwrap();
    let file = frontmatter_re.replace_all(file, "$f");
    
    let lines: Vec<Vec<&str>> = file.trim().lines().map(|l| l.split(": ").collect()).collect();
    
    let mut title: String = "example title".to_string();
    let mut date: String = "2021-01-01".to_string();
    let mut tags: String = "tag1, tag2".to_string();
    
    for line in lines {
        match line[0] {
            "title" => title = line[1].to_string(),
            "date" => date = line[1].to_string(),
            "tags" => tags = line[1].to_string(),
            _ => eprintln!("That shouldn't happen, line: {:?}", line[0])
        }
    }
    
    (title, date, tags)
}