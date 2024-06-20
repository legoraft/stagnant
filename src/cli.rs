pub struct Args {
    pub file = String,
    
    #[arg(short, long)]
    pub list = String,
}

pub fn parse(args: Vec<String>) -> Args {
    
}