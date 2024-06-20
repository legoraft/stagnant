use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub file: String,
    
    #[arg(short, long)]
    pub list: String,
}

pub fn parse_args(args: Vec<String>) -> Args {
    let args = Args::parse();
    
    args
}