use clap::Parser;

use libcerts::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    cli.exec()
}
