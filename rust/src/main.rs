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
        #[arg(short = 'r', long)]
        eruption_rate: String,

        /// Length of an eruption
        #[arg(short = 'd', long)]
        eruption_duration: String,

        /// Length of the eruption cycle
        #[arg(short = 'p', long)]
        eruption_period: String,

        /// Length of the activity period
        #[arg(short = 'D', long)]
        active_duration: String,

        /// Length of the whole activity/dormancy cycle
        #[arg(short = 'P', long)]
        active_period: String,
    },
    /// Print geyser types
    GeyserTypes,
    /// Convert mass to kilograms
    Kilo {
        /// The amount of mass to convert to a kilogram
        mass: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Geyser {
            kind,
            eruption_rate,
            eruption_duration,
            eruption_period,
            active_duration,
            active_period,
        }) => geyser::geyser_main(
            kind,
            eruption_rate,
            eruption_duration,
            eruption_period,
            active_duration,
            active_period,
        ),
        Some(Commands::Kilo { mass }) => oni_tools::read(mass.clone()),
        Some(Commands::GeyserTypes) => geyser::print_geyser_types(),
        None => {
            Cli::command().print_help().expect("Failed to print help.");
        }
    }
}
