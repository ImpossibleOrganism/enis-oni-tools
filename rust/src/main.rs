use std::env;

use clap::{CommandFactory, Parser, Subcommand};

use oni_tools;
use oni_tools::geyser;

#[derive(Parser)]
#[command(version, about = "Eni's Oxygen Not Included Tools")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run calculations for a geyser
    Geyser {
        /// Geyser type
        #[arg(short, long)]
        kind: String,

        /// Emission rate while erupting
        #[arg(short, long)]
        rate: f32,
    },
    /// Convert mass to kilograms
    Kilo {
        /// The amount of mass to convert to a kilogram
        mass: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Geyser { kind, rate }) => geyser::geyser_main(),
        Some(Commands::Kilo { mass }) => return oni_tools::read(mass.clone()),
        None => {
            Cli::command().print_help().expect("Failed to print help.");
        }
    }
}
