use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Convert an image.
    Image {
        /// The path to the image to convert to ascii.
        #[arg(short, long)]
        path: String,

        /// The width (in characters) of the resulting ASCII art. If only the height is provided, this one will be inferred while trying to maintain the best proportions possible. If neither is, the image's height and width will be used.
        #[arg(short = 'W', long)]
        width: Option<u32>,

        /// The height (in characters) of the resulting ASCII art. If only the width is provided, this one will be inferred while trying to maintain the best proportions possible. If neither is, the image's height and width will be used.
        #[arg(short = 'H', long)]
        height: Option<u32>,

        /// Whether to invert the image. (dark areas become lighter and vice-versa).
        #[arg(long)]
        invert: bool,

        /// The savepath for the ASCII art.
        #[arg(short, long)]
        savepath: Option<String>,

        /// Whether the ASCII art should also contain colors. Attention: colors are encoded in ANSI, be sure to use a text editor or terminal capable of displaying ANSI characters correctly.
        #[arg(short, long)]
        colored: bool,

        /// Only available when the "colored" flag is specified; makes it so every character is the most luminous one.
        #[arg(short = 'u', long = "uniform-char", requires = "colored")]
        uniform_char: bool,

        /// Disable parallelized operations while converting the image to ASCII art.
        #[arg(long = "no-parallel")]
        no_parallel: bool,
    },

    Video {
        /// The path to the video to convert to ascii.
        #[arg(short, long)]
        path: String,

        /// The width (in characters) of the resulting ASCII art. If only the height is provided, this one will be inferred while trying to maintain the best proportions possible. If neither is, the image's height and width will be used.
        #[arg(short = 'W', long)]
        width: Option<u32>,

        /// The height (in characters) of the resulting ASCII art. If only the width is provided, this one will be inferred while trying to maintain the best proportions possible. If neither is, the image's height and width will be used.
        #[arg(short = 'H', long)]
        height: Option<u32>,

        /// The number of total frames in the ASCII art video. The default is the original's video.
        #[arg(short = 'f', long = "number-frames")]
        n_frames: Option<usize>,

        /// Whether to invert the video. (dark areas become lighter and vice-versa).
        #[arg(long)]
        invert: bool,

        /// The savepath to the folder for ASCII art. If not specified, the ASCII video will be show in-terminal.
        #[arg(short, long, group = "frame-group")]
        savepath: Option<String>,

        /// The delay between one frame and the other, in ms (milliseconds).
        #[arg(long = "delay-frames", group = "frame-group")]
        delay_frames: Option<u32>,

        /// Whether the ASCII art should also contain colors. Attention: colors are encoded in ANSI, be sure to use a text editor or terminal capable of displaying ANSI characters correctly.
        #[arg(short, long)]
        colored: bool,

        /// Only available when the "colored" flag is specified; makes it so every character is the most luminous one.
        #[arg(short = 'u', long = "uniform-char", requires = "colored")]
        uniform_char: bool,

        /// Disable parallelized operations while converting the video to ASCII art.
        #[arg(long = "no-parallel")]
        no_parallel: bool,
    },

    /// Creates an ASCII art of the waveform of the provided audio.
    Audio {
         /// The path to the image to convert to ascii.
         #[arg(short, long)]
         path: String,
 
         /// The height (in characters) of the highest(or lowest) waveform peak in the resulting ASCII art. If not provided, standard is 255 characters.
         #[arg(short = 'H', long)]
         height: Option<u32>,
 
         /// Whether to invert the image. (dark areas become lighter and vice-versa).
         #[arg(long)]
         invert: bool,
 
         /// The savepath for the ASCII art.
         #[arg(short, long)]
         savepath: Option<String>,
 
         /// Makes it so every character is the most luminous one.
         #[arg(short = 'u', long = "uniform-char")]
         uniform_char: bool,
 
         /// Disable parallelized operations while converting the audio to ASCII art.
         #[arg(long = "no-parallel")]
         no_parallel: bool,

         /// specifies the audio type. (eg: mp3, ogg, etc...)
         #[arg(long, short)]
         media_type: String,
    },

    /// Read a folder with frames created by the video command, and then show them in the terminal.
    Read {
        /// Path to the folder to read the frames from
        #[arg(long, short)]
        path: String,

        /// Disable parallelized operations while reading the frames.
        #[arg(long = "no-parallel")]
        no_parallel: bool,

        /// The delay between one frame and the other while displaying them, in millisecods (ms). default is 100 ms.
        #[arg(long = "frame-delay", short)]
        frame_delay: Option<usize>,
    }
}
