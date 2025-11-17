//! passgen CLI utility
//! Generated with ChatGPT

mod generator;
mod words;

use clap::{Parser, Subcommand};
use generator::*;

#[derive(Parser)]
#[command(name = "passgen")]
#[command(about = "Fast, strong password generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Default mode: secure passphrase
    Default,

    /// N random chars (full charset)
    Chars {
        /// Length
        n: usize,
    },

    /// N random chars (letters + digits)
    CharsNs {
        /// Length
        n: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Default => generate_default(),
        Commands::Chars { n } => generate_chars(n),
        Commands::CharsNs { n } => generate_chars_no_special(n),
    };

    print!("{}", output);
}
