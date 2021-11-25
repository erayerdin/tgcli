use fern::colors::{Color, ColoredLevelConfig};

// Copyright 2021 Eray Erdin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub fn set_logger(
    // Defines verbosity level.
    // 0 - Info, Warn, Error
    // 1 - Debug, Info, Warn, Error + Level Labels
    // 2 - Debug, Info, Warn, Error + Level Labels + Location Labels
    // 3 - Trace, Debug, Info, Warn, Error + Level Labels + Location Labels
    verbosity: u64,
) -> Result<(), log::SetLoggerError> {
    let colors = ColoredLevelConfig::new()
        .error(Color::BrightRed)
        .warn(Color::Yellow)
        .debug(Color::Blue)
        .trace(Color::Cyan);

    fern::Dispatch::new()
        // stdout chain
        .chain(
            fern::Dispatch::new()
                .filter(|metadata| {
                    metadata.level() != log::LevelFilter::Error
                        || metadata.level() != log::LevelFilter::Warn
                })
                .format(move |out, message, record| {
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
                    match verbosity {
                        1 => out.finish(format_args!(
                            "\x1B[{}m[{}] {}\x1B[0m",
                            colors.get_color(&record.level()).to_fg_str(),
                            record.level(),
                            message
                        )),
                        2..=u64::MAX => out.finish(format_args!(
                            "\x1B[{}m[{}][{}] {}\x1B[0m",
                            colors.get_color(&record.level()).to_fg_str(),
                            record.level(),
                            record.target(),
                            message
                        )),
                        0 => out.finish(format_args!(
                            "\x1B[{}m{}\x1B[0m",
                            colors.get_color(&record.level()).to_fg_str(),
                            message
                        )),
                    }
                })
                .level(
                    // if cfg!(debug_assertions) {
                    //     log::LevelFilter::Trace
                    // } else {
                    //     log::LevelFilter::Info
                    // }
                    match verbosity {
                        0 => log::LevelFilter::Info,
                        1 | 2 => log::LevelFilter::Debug,
                        _ => log::LevelFilter::Trace,
                    },
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
                .format(move |out, message, record| match verbosity {
                    1 => out.finish(format_args!(
                        "\x1B[{}m[{}] {}\x1B[0m",
                        colors.get_color(&record.level()).to_fg_str(),
                        record.level(),
                        message
                    )),
                    2..=u64::MAX => out.finish(format_args!(
                        "\x1B[{}m[{}][{}] {}\x1B[0m",
                        colors.get_color(&record.level()).to_fg_str(),
                        record.level(),
                        record.target(),
                        message
                    )),
                    0 => out.finish(format_args!(
                        "\x1B[{}m{}\x1B[0m",
                        colors.get_color(&record.level()).to_fg_str(),
                        message
                    )),
                })
                .level(log::LevelFilter::Warn)
                .chain(std::io::stderr()),
        )
        .apply()
}
