mod commands {
    pub mod analyze;
    pub mod dedup;
}

mod models {
    pub mod filter;
}

mod utils {
    pub mod url;
}

use clap::{Parser, Subcommand};
mod errors;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Shows duplicate filters in a JSON file.")]
    Dedup {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long, help = "Treats file as live filter data")]
        live: bool,
    },

    #[command(about = "Analyzes a JSON file containing filter data and outputs the best filters.")]
    Analyze {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long, default_value_t = 10)]
        count: usize,

        #[arg(short, long, help = "Opens in default web browser")]
        open: bool,

        #[arg(short, long, help = "Treats file as live filter data")]
        live: bool,
    },
}

fn main() {
    let cli = CLI::parse();
    match &cli.command {
        Commands::Analyze {
            filename,
            count,
            open,
            live,
        } => {
            if let Err(err) = commands::analyze::run(filename, *count, *open, *live) {
                eprintln!("Failed to run analysis: {}", err);
            }
        }
        Commands::Dedup { filename, live } => {
            if let Err(err) = commands::dedup::run(filename, *live) {
                eprintln!("Failed to run deduplication: {}", err);
            }
        }
    }
}
