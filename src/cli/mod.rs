use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, AppSettings,
    Arg, SubCommand,
};

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

pub fn get_app() -> App<'static, 'static> {
    #[allow(non_snake_case)]
    let MESSAGE_ARG = Arg::with_name("message")
        .long("message")
        .short("m")
        .takes_value(true)
        .help("A message to be sent.")
        .validator(|v| {
            // max 1024 characters
            todo!()
        });
    #[allow(non_snake_case)]
    let FILE_ARG = Arg::with_name("file")
        .required(true)
        .help("A file to be uploaded.")
        .validator(|v| todo!());

    app_from_crate!()
        .global_settings(&[
            AppSettings::ColorAlways,
            AppSettings::ColoredHelp,
            AppSettings::GlobalVersion,
            AppSettings::DeriveDisplayOrder,
        ])
        .settings(&[AppSettings::SubcommandRequiredElseHelp])
        .args(&[
            Arg::with_name("secure")
                .long("secure")
                .help("To provide secure connection. This is the default behavior.")
                .conflicts_with("no-secure"),
            Arg::with_name("no-secure")
                .long("no-secure")
                .help(r#"Reverse of "secure" flag."#),
        ])
        .subcommands(vec![SubCommand::with_name("bot")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .about("Operations for bots.")
            .args(&[Arg::with_name("token")
                .long("token")
                .short("t")
                .help("Telegram bot token.")
                .takes_value(true)
                .required(true)
                .env("TELEGRAM_BOT_TOKEN")
                .hide_env_values(true)])
            .subcommands(vec![SubCommand::with_name("send")
                .settings(&[AppSettings::SubcommandRequiredElseHelp])
                .about("Sending operations for bots.")
                .args(&[
                    Arg::with_name("receiver")
                        .help("The chat ID of receiver.")
                        .short("r")
                        .takes_value(true)
                        .required(true),
                    Arg::with_name("format")
                        .long("format")
                        .help("Format of the message.")
                        .takes_value(true)
                        .possible_values(&["markdown", "html"])
                        .default_value("markdown"),
                ])
                .subcommands(vec![
                    SubCommand::with_name("message")
                        .about("Send a message with a bot.")
                        .args(&[Arg::with_name("message")
                            .help("A message to be sent.")
                            .required(true)]),
                    SubCommand::with_name("document")
                        .about("Send a document with a bot.")
                        .args(&[
                            FILE_ARG.clone(),
                            MESSAGE_ARG.clone(),
                            Arg::with_name("thumbnail")
                                .long("thumbnail")
                                .help("A thumbnail for the document.")
                                .required(true)
                                .takes_value(true)
                                .validator(|v| todo!()),
                        ]),
                    SubCommand::with_name("photo")
                        .about("Send a photo with a bot.")
                        .args(&[FILE_ARG.clone(), MESSAGE_ARG.clone()]),
                    SubCommand::with_name("video")
                        .about("Send a video with a bot.")
                        .args(&[
                            FILE_ARG.clone(),
                            MESSAGE_ARG.clone(),
                            Arg::with_name("horizontal")
                                .help("Horizontal aspect ratio of the video.")
                                .takes_value(true)
                                .short("h")
                                .default_value("1")
                                .validator(|v| todo!()),
                            Arg::with_name("vertical")
                                .help("Vertical aspect ratio of the video.")
                                .takes_value(true)
                                .short("v")
                                .default_value("1")
                                .validator(|v| todo!()),
                        ]),
                    SubCommand::with_name("audio")
                        .about("Send an audio with a bot.")
                        .args(&[
                            FILE_ARG.clone(),
                            MESSAGE_ARG.clone(),
                            Arg::with_name("performer")
                                .long("performer")
                                .help("The performer of the audio.")
                                .takes_value(true),
                            Arg::with_name("title")
                                .long("title")
                                .help("The title of the audio.")
                                .takes_value(true),
                        ]),
                    SubCommand::with_name("poll")
                        .about("Send a poll with a bot.")
                        .args(&[
                            Arg::with_name("question")
                                .help("The question to ask.")
                                .short("q")
                                .takes_value(true)
                                .required(true),
                            Arg::with_name("option")
                                .help("An option for the question.")
                                .short("o")
                                .multiple(true)
                                .min_values(1),
                        ]),
                    SubCommand::with_name("location")
                        .about("Send a location with a bot.")
                        .args(&[
                            Arg::with_name("latitude")
                                .help("The latitude of the location.")
                                .short("x")
                                .takes_value(true)
                                .required(true),
                            Arg::with_name("longitude")
                                .help("The longitude of the location.")
                                .short("y")
                                .takes_value(true)
                                .required(true),
                        ]),
                ])])])
}

pub fn match_app(app: App) {
    app.get_matches();
}