use std::process::exit;

use tgcli::cli::{get_app, match_app};

fn main() {
    let app = get_app();
    match match_app(app) {
        Ok(_) => exit(0),
        Err(e) => e.exit(),
    }
}
