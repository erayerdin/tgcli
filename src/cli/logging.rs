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

pub fn get_logger() -> Result<(), log::SetLoggerError> {
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
}
