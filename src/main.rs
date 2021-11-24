use std::process::exit;

use tgcli::cli::{get_app, match_app};

fn main() {
    fern::Dispatch::new()
        // stdout chain
        .chain(
            fern::Dispatch::new()
                .filter(|metadata| {
                    metadata.level() != log::LevelFilter::Error
                        || metadata.level() != log::LevelFilter::Warn
                })
                .format(|out, message, _record| {
                    // if cfg!(debug_assertions) {
                    //     out.finish(format_args!(
                    //         "[{}][{}] {}",
                    //         record.level(),
                    //         record.target(),
                    //         message
                    //     ))
                    // } else {
                    //     match record.level() {
                    //         log::Level::Error => out.finish(format_args!("Error: {}", message)),
                    //         log::Level::Warn => out.finish(format_args!("Warning: {}", message)),
                    //         _ => out.finish(format_args!("{}", message)),
                    //     }
                    // }
                    out.finish(format_args!("{}", message))
                })
                .level(
                    // if cfg!(debug_assertions) {
                    //     log::LevelFilter::Trace
                    // } else {
                    //     log::LevelFilter::Info
                    // }
                    log::LevelFilter::Info,
                )
                .chain(std::io::stdout()),
        )
        // stderr chain
        .chain(
            fern::Dispatch::new()
                .filter(|metadata| {
                    metadata.level() == log::LevelFilter::Error
                        || metadata.level() == log::LevelFilter::Warn
                })
                .format(|out, message, _record| out.finish(format_args!("{}", message)))
                .level(log::LevelFilter::Info)
                .chain(std::io::stderr()),
        )
        .apply()
        .unwrap();

    let app = get_app();
    match match_app(app) {
        Ok(_) => exit(0),
        Err(e) => e.exit(),
    }
}
