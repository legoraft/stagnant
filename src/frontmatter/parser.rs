use regex::Regex;

pub fn parse(file: &str) -> (&str, &str, Vec<&str>) {
    let frontmatter_re = Regex::new(r"(?s)---(?<f>.*?)---(.*)").unwrap();
    let file = frontmatter_re.replace_all(file, "$f");
    
    let lines: Vec<Vec<&str>> = file.lines().map(|l| l.split(": ").collect()).collect();
    
    let mut title: &str = "example title";
    let mut date: &str = "2021-01-01";
    let mut tags: Vec<&str> = vec!["tag1", "tag2"];
    
    for line in lines {
        match line[0] {
            "title" => title = line[1],
            "date" => date = line[1],
            "tags" => tags = line[1].split(" ").collect(),
            _ => eprintln!("That shouldn't happen, line: {:?}", line[0])
        }
    }
    
    (title, date, tags)
}