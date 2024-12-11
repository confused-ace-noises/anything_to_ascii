use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub path: String,

    #[arg(short, long)]
    pub vertical: Option<usize>,
    
    #[arg(short = 'o', long)]
    pub horizontal: Option<usize>,

    #[arg(long)]
    pub invert: bool,

    #[arg(short, long)]
    pub savepath: Option<String>,
}