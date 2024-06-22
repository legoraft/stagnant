use clap::Parser;

// Generates simple cli commands, long and short and a help screen.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub directory: Option<String>,
    #[arg(short, long)]
    pub list_file: Option<String>,
}