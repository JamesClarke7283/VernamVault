
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vernamvault", version = "1.0", about = "A tool for one-time pad encryption and decryption")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate keys
    Generate {
        /// Length of the key in UTF-8 characters plus 16 bytes for the identifier
        #[arg(short, long)]
        key_length: usize,

        /// Number of keys to generate
        #[arg(short, long)]
        key_count: usize,

        /// Directory where the keys will be stored
        #[arg(short, long)]
        output_dir: PathBuf,
    },
    /// Encrypt a message
    Encrypt {
        /// The message to encrypt
        #[arg(short, long, required_unless_present = "input_file")]
        message: Option<String>,

        /// The input file containing the message to encrypt
        #[arg(short, long, required_unless_present = "message")]
        input_file: Option<PathBuf>,

        /// Directory where the keys are stored
        #[arg(short, long)]
        key_dir: PathBuf,

        /// Output file for the encrypted message
        #[arg(short, long)]
        output_file: Option<PathBuf>,
    },
    /// Decrypt a message
    Decrypt {
        /// Directory where the keys are stored
        #[arg(short, long)]
        key_dir: PathBuf,

        /// Input file containing the ciphertext
        #[arg(short, long)]
        input_file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate {
            key_length,
            key_count,
            output_dir,
        } => {
            println!(
                "Generating {} keys of length {} in directory {:?}",
                key_count, key_length, output_dir
            );
        }
        Commands::Encrypt {
            message,
            input_file,
            key_dir,
            output_file,
        } => {
            if let Some(message) = message {
                println!("Encrypting message: {}", message);
            } else if let Some(input_file) = input_file {
                println!("Encrypting message from file: {:?}", input_file);
            }
            println!("Using keys from directory: {:?}", key_dir);
            if let Some(output_file) = output_file {
                println!("Outputting encrypted message to file: {:?}", output_file);
            } else {
                println!("Outputting encrypted message to stdout");
            }
        }
        Commands::Decrypt {
            key_dir,
            input_file,
        } => {
            println!("Decrypting message from file: {:?}", input_file);
            println!("Using keys from directory: {:?}", key_dir);
        }
    }
}

