use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub directory: Option<String>,
    #[arg(short, long)]
    pub list_file: Option<String>,
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    
    args
}