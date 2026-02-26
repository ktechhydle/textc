use clap::{CommandFactory, Parser, Subcommand};
use textc::{read_and_compress, read_and_decompress};

#[derive(Parser)]
#[command(name = "textc", version = env!("CARGO_PKG_VERSION"), about = "Compression and Decompression for UTF-8 Text Files")]
struct Cli {
    file: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "-c", about = "Compress a text file into the .tzp format")]
    Compress { output_file: String },
    #[command(name = "-d", about = "Decompress a .tzp file into plain text")]
    Decompress { output_file: String },
}

fn main() {
    let cli = Cli::parse();

    match (cli.command, cli.file) {
        (Some(Commands::Compress { output_file }), Some(file)) => {
            read_and_compress(&file, &output_file);
        }
        (Some(Commands::Decompress { output_file }), Some(file)) => {
            read_and_decompress(&file, &output_file);
        }
        (_, _) => {
            Cli::command().print_help().unwrap();
        }
    }
}
