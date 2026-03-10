use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "hgignore-to-gitignore")]
#[command(version = "0.1.0")]
#[command(author = "Denis Salmanovich")]
#[command(about = "Convert .hgignore files to .gitignore format", long_about = None)]
pub struct Args {
    /// Remove .hgignore file after conversion
    #[arg(short, long)]
    pub replace_and_remove: bool,
}
