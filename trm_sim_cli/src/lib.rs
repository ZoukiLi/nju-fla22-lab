mod cli;
mod trm_wrapper;

use clap::Parser;
pub use cli::Cli;
pub use trm_wrapper::MachineWrapper;

pub fn run() {
    let cli = Cli::parse();
    let mut machine = MachineWrapper::from_file(&cli.file, cli.ext.as_deref()).unwrap();
    let input = cli.input.unwrap_or_else(|| {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s
    });
    let output = machine.run(&input, cli.verbose).unwrap();
    println!("{}", output);
}