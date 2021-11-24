use std::process::exit;

use tgcli::cli::{get_app, logging::get_logger, match_app};

fn main() {
    get_logger().unwrap();

    let app = get_app();
    match match_app(app) {
        Ok(_) => exit(0),
        Err(e) => e.exit(),
    }
}
