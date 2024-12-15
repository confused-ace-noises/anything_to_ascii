use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The path to the image to convert to ascii.
    #[arg(short, long)]
    pub path: String,

    /// The number of vertical characters. If only the horizontal number of pixels is provided, this one will be inferred to try to maintain the best proportions possible.
    #[arg(short, long)]
    pub vertical: Option<usize>,
    
    /// The number of horizontal characters. If only the vertical number of pixels is provided, this one will be inferred to try to maintain the best proportions possible.
    #[arg(short = 'o', long)]
    pub horizontal: Option<usize>,

    /// Whether to invert the image. (dark areas become lighter and vice-versa)
    #[arg(long)]
    pub invert: bool,

    /// The savepath for the ASCII art.
    #[arg(short, long)]
    pub savepath: Option<String>,

    /// Whether the ASCII art should also contain colors. Attention: colors are encoded in ANSI, be sure to use a text editor or terminal capable of displaying ANSI characters correctly.
    #[arg(short, long)]
    pub colored: bool,

    /// only available when the "colored" flag is specified; makes it so every character is the most luminous one.
    #[arg(short = 'u', long = "uniform-char", requires = "colored")]
    pub uniform_char: bool,
}