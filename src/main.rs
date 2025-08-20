use clap::{Parser, Subcommand};

mod commands;

pub use ipchecker::ip_utils;

#[derive(Parser)]
#[command(version, about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    /// Show detailed information
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check if an IP address belongs to a known crawler
    Crawler {
        /// IP address to check
        ip_address: String,
    },
    /// Check CIDR network overlap
    Cidr {
        /// First CIDR network (e.g., 192.168.1.0/24)
        network1: String,
        /// Second CIDR network (e.g., 192.168.0.0/16)
        network2: String,
    },
    /// Check country code for an IP address
    Cc {
        /// IP address to check
        ip_address: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Crawler { ip_address } => {
            commands::crawler::check_crawler(&ip_address, cli.verbose)
        }
        Commands::Cidr { network1, network2 } => {
            commands::cidr::check_cidr_overlap(&network1, &network2, cli.verbose)
        }
        Commands::Cc { ip_address } => commands::cc::check_country_code(&ip_address, cli.verbose),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
