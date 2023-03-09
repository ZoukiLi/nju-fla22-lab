mod cli;
mod trm_wrapper;

use clap::Parser;
pub use cli::Cli;
pub use trm_wrapper::MachineWrapper;

pub fn run() {
    let cli = Cli::parse();
    let mut machine = MachineWrapper::from_file(&cli.file, cli.ext.as_deref()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let mut s = String::new();
    let input = cli.input.as_deref().unwrap_or_else(|| {
        std::io::stdin().read_line(&mut s).unwrap_or_else(|_| {
            eprintln!("Failed to read from stdin");
            std::process::exit(1);
        });
        // remove trailing newline
        s.trim()
    });

    let output = machine.run(input, cli.verbose).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    println!("{}", output);
}