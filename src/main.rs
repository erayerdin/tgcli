use std::process;

use log::error;
use tgcli::cli::{get_app, match_app};

fn main() {
    let app = get_app();
    match match_app(app) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
            process::exit(e.exit_code());
        }
    }
}
