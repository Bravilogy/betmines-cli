mod commands {
    pub mod analyze;
    pub mod cleanup;
    pub mod dedup;
    pub mod outcomes;
}

mod models {
    pub mod filter;
    pub mod filter_traits;
}

mod services {
    pub mod filter_service;
}

mod utils {
    pub mod command;
    pub mod config;
    pub mod filesystem;
    pub mod logging;
    pub mod paths;
}

use clap::{Parser, Subcommand};
use commands::cleanup;
use utils::logging;
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
        filename: Option<String>,

        #[arg(short, long, help = "Treats file as live filter data")]
        live: bool,
    },

    #[command(about = "Renames filters' serviceNames based on their desiredOutcome values")]
    Outcomes {
        #[arg(short, long, help = "Evaluates live filters")]
        live: bool,

        #[arg(short, long, help = "Run without making actual changes")]
        dry_run: bool,
    },

    #[command(about = "Remove low performing filters from betmines")]
    Cleanup {
        #[arg(short, long, help = "Evaluates live filters")]
        live: bool,
    },

    #[command(about = "Analyzes a JSON file containing filter data and outputs the best filters.")]
    Analyze {
        #[arg(short, long, help = "Path to the JSON file")]
        filename: Option<String>,

        #[arg(long, help = "Existing filters to compare against.")]
        existing: Option<String>,

        #[arg(long, default_value_t = 10)]
        count: usize,

        #[arg(long, help = "Opens in default web browser")]
        open: bool,

        #[arg(long, help = "Automatically imports the best filters")]
        autoimport: bool,

        #[arg(long, help = "Treats file as live filter data")]
        live: bool,

        #[arg(
            long,
            default_value_t = 0,
            help = "Offsets the count by supplied value"
        )]
        offset: usize,

        #[arg(short, long, help = "Prints verbose output")]
        verbose: bool,
    },
}

fn main() {
    logging::setup_logging();

    if let Err(err) = utils::command::fetch_filters() {
        log::error!("Failed to fetch filters: {}", err);
    }

    let cli = CLI::parse();

    match &cli.command {
        Commands::Outcomes { live, dry_run } => {
            log::info!(
                "Running outcomes renaming for {} filters{}",
                if *live { "live" } else { "pre-match" },
                if *dry_run { " (dry-run)" } else { "" },
            );

            if let Err(err) = commands::outcomes::run(*live, *dry_run) {
                log::error!("Failed to run outcomes command: {}", err);
            }
        }
        Commands::Analyze {
            filename,
            existing,
            count,
            open,
            live,
            offset,
            autoimport,
            verbose,
        } => {
            let file_path = filename
                .clone()
                .unwrap_or_else(|| utils::paths::get_data_path(*live).to_string());

            log::info!("Running analysis on {} for {} filters", file_path, count);

            if let Err(err) = commands::analyze::run(
                file_path,
                existing,
                *count,
                *open,
                *live,
                *offset,
                *autoimport,
                *verbose,
            ) {
                log::error!("Failed to run analysis: {}", err);
            }
        }
        Commands::Cleanup { live } => {
            log::info!(
                "Running cleanup for {} filters",
                if *live { "live" } else { "pre-match" }
            );

            if let Err(err) = cleanup::run(*live) {
                log::error!("Failed to run cleanup: {}", err);
            }
        }
        Commands::Dedup { filename, live } => {
            let file_path = filename
                .clone()
                .unwrap_or_else(|| utils::paths::get_existing_path(*live).to_string());

            log::info!("Running deduplication on {}", file_path);

            if let Err(err) = commands::dedup::run(file_path, *live) {
                log::error!("Failed to run deduplication: {}", err);
            }
        }
    }
}
