use clap::Parser;
use std::fs;
use std::path::Path;
use std::process;

mod converter;
mod cli;

use cli::Args;
use converter::HgIgnoreConverter;

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(message) => {
            println!("{}", message);
            process::exit(0);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }
}

fn run(args: Args) -> Result<String, String> {
    let hgignore_path = Path::new(".hgignore");
    let gitignore_path = Path::new(".gitignore");

    // Check if .hgignore exists
    if !hgignore_path.exists() {
        return Err(
            "File '.hgignore' not found in current directory.\n\
            Please ensure you run this command from the directory containing the .hgignore file."
                .to_string(),
        );
    }

    // Read .hgignore file
    let hgignore_content = fs::read_to_string(hgignore_path)
        .map_err(|e| format!("Failed to read .hgignore: {}", e))?;

    // Convert content
    let converter = HgIgnoreConverter::new();
    let gitignore_content = converter.convert(&hgignore_content)?;

    // Write .gitignore file
    fs::write(gitignore_path, &gitignore_content)
        .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

    let message = "Successfully converted .hgignore to .gitignore".to_string();

    // Remove .hgignore if --replace_and_remove or -r flag is set
    if args.replace_and_remove {
        fs::remove_file(hgignore_path)
            .map_err(|e| format!("Failed to remove .hgignore: {}", e))?;
        return Ok(format!("{} and removed .hgignore", message));
    }

    Ok(message)
}