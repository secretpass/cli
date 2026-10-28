/**
* To keep some of our secrets to ourselves for IP protection,
* We've opted to keep a majority of the logic in a private core package that this package inherits from
* Over time we do intend to opensource pieces of the core, or the full core logic.
**/
use clap::Parser;
use secretpass_local::cli;

fn main() {
    let spass_cli = cli::Cli::parse();

    spass_cli.execute()
}
