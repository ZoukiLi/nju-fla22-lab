use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path for turing machine definition file
    #[arg(short, long)]
    pub file: String,

    /// The extension of the file, if not provided, will be inferred from the file path.
    /// Now only supports [json, yaml, toml]
    #[arg(short, long)]
    pub ext: Option<String>,

    /// If provided, the machine will be run in verbose mode, every step will be printed.
    #[arg(short, long)]
    pub verbose: bool,

    /// The input string for the machine, if not provided, will be read from stdin.
    #[arg(short, long)]
    pub input: Option<String>,
}