use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "cipherant")]
#[command(about = "Personal Research Agent", long_about = None)]
pub struct Cli {
    pub prompt: String,
}
