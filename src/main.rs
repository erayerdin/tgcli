use tgcli::cli::{get_app, match_app};

fn main() {
    let app = get_app();
    match_app(app);
}
