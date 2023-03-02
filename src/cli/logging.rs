use fern::colors::{Color, ColoredLevelConfig};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) fn set_logger(
    // Defines verbosity level.
    // 0 - Info, Warn, Error + Self Target
    // 1 - Debug, Info, Warn, Error + Self Target + Level Labels
    // 2 - Debug, Info, Warn, Error + Self Target + Level Labels + Location Labels
    // 3 - Trace, Debug, Info, Warn, Error + Self Target + Level Labels + Location Labels
    // 4 - Trace, Debug, Info, Warn, Error + All Targets + Level Labels + Location Labels
    verbosity_level: &clap_verbosity_flag::Verbosity,
) -> Result<(), log::SetLoggerError> {
    let verbosity: u8 = match verbosity_level.log_level_filter() {
        log::LevelFilter::Off => 0,
        log::LevelFilter::Trace => 1,
        log::LevelFilter::Debug => 2,
        log::LevelFilter::Info => 3,
        log::LevelFilter::Warn => 4,
        log::LevelFilter::Error => 5,
    };

    let colors = ColoredLevelConfig::new()
        .error(Color::BrightRed)
        .warn(Color::Yellow)
        .debug(Color::Blue)
        .trace(Color::Cyan);

    fern::Dispatch::new()
        .filter(move |metadata| {
            if verbosity >= 4 {
                true
            } else {
                metadata.target().starts_with("tgcli")
            }
        })
        // stdout chain
        .chain(
            fern::Dispatch::new()
                .filter(|metadata| {
                    metadata.level() != log::LevelFilter::Error
                        || metadata.level() != log::LevelFilter::Warn
                })
                .format(move |out, message, record| match verbosity {
                    1 => out.finish(format_args!(
                        "[{levelname}] {message}",
                        levelname = record.level(),
                        message = message,
                    )),
                    2..=u8::MAX => out.finish(format_args!(
                        "[{levelname}][{targetname}] {message}",
                        levelname = record.level(),
                        targetname = record.target(),
                        message = message,
                    )),
                    0 => out.finish(format_args!(
                        "\x1B[{}m{}\x1B[0m",
                        colors.get_color(&record.level()).to_fg_str(),
                        message
                    )),
                })
                .level(match verbosity {
                    0 => log::LevelFilter::Info,
                    1 | 2 => log::LevelFilter::Debug,
                    _ => log::LevelFilter::Trace,
                })
                .chain(std::io::stdout()),
        )
        // stderr chain
        .chain(
            fern::Dispatch::new()
                .filter(|metadata| {
                    metadata.level() == log::LevelFilter::Error
                        || metadata.level() == log::LevelFilter::Warn
                })
                .format(move |out, message, record| match verbosity {
                    1 => out.finish(format_args!(
                        "[{levelname}] {message}",
                        levelname = record.level(),
                        message = message,
                    )),
                    2..=u8::MAX => out.finish(format_args!(
                        "[{levelname}][{targetname}] {message}",
                        levelname = record.level(),
                        targetname = record.target(),
                        message = message,
                    )),
                    0 => out.finish(format_args!(
                        "\x1B[{colorbyte}m{message}\x1B[0m",
                        colorbyte = colors.get_color(&record.level()).to_fg_str(),
                        message = message,
                    )),
                })
                .level(log::LevelFilter::Warn)
                .chain(std::io::stderr()),
        )
        .apply()
}
