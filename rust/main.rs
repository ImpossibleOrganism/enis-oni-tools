use clap::{CommandFactory, Parser, Subcommand};

use oni_tools;
use oni_tools::geyser;
use oni_tools::Parseable;

#[derive(Parser)]
#[command(version, about = "Eni's Oxygen Not Included Tools")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Geyser-related commands
    #[command(subcommand)]
    Geyser(GeyserCommands),
    /// Convert mass to kilograms
    Kilo {
        /// The amount of mass to convert to a kilogram
        mass: String,
    },
}

#[derive(Subcommand)]
enum GeyserCommands {
    /// Run rate calculations for a geyser
    Yield {
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
    Types,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Geyser(command)) => match command {
            GeyserCommands::Yield {
                kind,
                eruption_rate,
                eruption_duration,
                eruption_period,
                active_duration,
                active_period,
            } => geyser::print_geyser_yield(
                kind,
                eruption_rate,
                eruption_duration,
                eruption_period,
                active_duration,
                active_period,
            ),
            GeyserCommands::Types => geyser::print_geyser_types(),
        },
        Some(Commands::Kilo { mass }) => println!("{}", oni_tools::convert(Parseable::Parse(mass))),
        None => {
            Cli::command().print_help().expect("Failed to print help.");
        }
    }
}
