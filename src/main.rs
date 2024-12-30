use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Converts a Base58 string to a byte array JSON file"
)]
struct Args {
    /// The Base58 string to convert
    #[arg(short, long, group = "input_source")]
    input: Option<String>,

    /// Input file containing the Base58 string
    #[arg(short = 'f', long, group = "input_source")]
    input_file: Option<PathBuf>,

    /// Output JSON file path
    #[arg(short, long, default_value = "output.json")]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    // Get input string either directly or from file
    let input_string = match (args.input, args.input_file) {
        (Some(input), None) => input,
        (None, Some(file_path)) => {
            pb.set_message("Reading input file...");
            let mut file = File::open(file_path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
            contents.trim().to_string()
        }
        _ => {
            return Err("Either --input or --input-file must be provided".into());
        }
    };

    // Decode Base58 string to bytes
    pb.set_message("Decoding Base58 string...");
    let output_bytes = bs58::decode(&input_string).into_vec().unwrap();
    pb.set_message("Base58 decoding completed");

    // Convert to JSON
    pb.set_message("Converting to JSON...");
    let json = serde_json::to_string(&output_bytes)?;
    pb.set_message("JSON conversion completed");

    // Write to file
    pb.set_message(format!("Writing to {}...", args.output.display()));
    let mut file = File::create(&args.output).await?;
    file.write_all(json.as_bytes()).await?;

    pb.finish_with_message(format!("✨ Successfully wrote output to {}", args.output.display()));
    Ok(())
}
