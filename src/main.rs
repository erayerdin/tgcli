use std::process;

use clap::StructOpt;
use log::error;
use tgcli::cli::{match_app, Cli};

fn main() {
    let cli = Cli::parse();
    match match_app(cli) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
            process::exit(e.exit_code());
        }
    }
}
