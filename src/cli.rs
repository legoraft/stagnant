use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub template: Option<String>,
    #[arg(short, long)]
    pub list: Option<String>,
}

pub fn parse_args(args: Vec<String>) -> Args {
    let args = Args::parse();
    
    args
}