use std::process::exit;

use tgcli::cli::{get_app, match_app};

#[tokio::main]
async fn main() {
    let app = get_app().await;
    match match_app(app).await {
        Ok(_) => exit(0),
        Err(e) => e.exit(),
    }
}
