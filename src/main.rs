use clap::Parser;

#[derive(Parser)]
#[command(version, about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
}
