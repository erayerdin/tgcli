use std::{convert::TryFrom, process};

use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, AppSettings,
    Arg, SubCommand,
};

use crate::{
    cli::validators::{
        audio_validator, caption_validator, file_validator, float_validator, image_validator,
        positive_integer_validator, question_validator, video_validator,
    },
    operations::{
        bot::send::{
            audio::SendAudioOperation, document::SendDocumentOperation,
            location::SendLocationOperation, message::SendMessageOperation,
            photo::SendPhotoOperation, poll::SendPollOperation, video::SendVideoOperation,
            SendOperation,
        },
        OperationError,
    },
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

pub mod validators;

pub fn get_app() -> App<'static, 'static> {
    #[allow(non_snake_case)]
    let CAPTION_ARG = Arg::with_name("message")
        .long("message")
        .short("m")
        .takes_value(true)
        .help("A message to be sent.")
        .validator(caption_validator);
    #[allow(non_snake_case)]
    let FILE_ARG = Arg::with_name("file")
        .required(true)
        .help("A file to be uploaded.")
        .validator(file_validator);

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
                // an arg cannot be global and required at the same time for some reason
                // REF https://github.com/clap-rs/clap/issues/1546
                // .required(true)
                .env("TELEGRAM_BOT_TOKEN")
                .hide_env_values(true)
                .global(true)])
            .subcommands(vec![SubCommand::with_name("send")
                .settings(&[AppSettings::SubcommandRequiredElseHelp])
                .about("Sending operations for bots.")
                .args(&[
                    Arg::with_name("receiver")
                        .short("r")
                        .long("receiver")
                        .help("The chat ID of receiver.")
                        .takes_value(true)
                        // an arg cannot be global and required at the same time for some reason
                        // REF https://github.com/clap-rs/clap/issues/1546
                        // .required(true)
                        .global(true),
                    Arg::with_name("format")
                        .long("format")
                        .help("Format of the message.")
                        .takes_value(true)
                        .possible_values(&["markdown", "html"])
                        .default_value("markdown")
                        .global(true),
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
                            CAPTION_ARG.clone(),
                            Arg::with_name("thumbnail")
                                .long("thumbnail")
                                .help("A thumbnail for the document.")
                                .takes_value(true)
                                .validator(image_validator),
                        ]),
                    SubCommand::with_name("photo")
                        .about("Send a photo with a bot.")
                        .args(&[
                            FILE_ARG.clone().validator(image_validator),
                            CAPTION_ARG.clone(),
                        ]),
                    SubCommand::with_name("video")
                        .about("Send a video with a bot.")
                        .args(&[
                            FILE_ARG.clone().validator(video_validator),
                            CAPTION_ARG.clone(),
                            Arg::with_name("horizontal")
                                .help("Horizontal aspect ratio of the video.")
                                .takes_value(true)
                                .short("h")
                                .default_value("1")
                                .validator(positive_integer_validator),
                            Arg::with_name("vertical")
                                .help("Vertical aspect ratio of the video.")
                                .takes_value(true)
                                .short("v")
                                .default_value("1")
                                .validator(positive_integer_validator),
                        ]),
                    SubCommand::with_name("audio")
                        .about("Send an audio with a bot.")
                        .args(&[
                            FILE_ARG.clone().validator(audio_validator),
                            CAPTION_ARG.clone(),
                            Arg::with_name("performer")
                                .long("performer")
                                .help("The performer of the audio.")
                                .takes_value(true),
                            Arg::with_name("title")
                                .long("title")
                                .help("The title of the audio.")
                                .takes_value(true),
                        ]),
                    // TODO validate characters have 1-100 characters
                    SubCommand::with_name("poll")
                        .about("Send a poll with a bot.")
                        .args(&[
                            Arg::with_name("question")
                                .help("The question to ask.")
                                .takes_value(true)
                                .required(true)
                                .validator(question_validator),
                            Arg::with_name("option")
                                .help("An option for the question.")
                                .short("o")
                                .required(true)
                                .multiple(true)
                                .min_values(2),
                        ]),
                    SubCommand::with_name("location")
                        .about("Send a location with a bot.")
                        .args(&[
                            Arg::with_name("latitude")
                                .help("The latitude of the location.")
                                .short("x")
                                .takes_value(true)
                                .required(true)
                                .validator(float_validator),
                            Arg::with_name("longitude")
                                .help("The longitude of the location.")
                                .short("y")
                                .takes_value(true)
                                .required(true)
                                .validator(float_validator),
                        ]),
                ])])])
}

pub fn match_app(app: App<'static, 'static>) -> Result<(), OperationError> {
    let matches = app.get_matches();

    match matches.subcommand() {
        ("bot", Some(bot_subc)) => match bot_subc.subcommand() {
            ("send", Some(send_subc)) => match send_subc.subcommand() {
                ("audio", Some(audio_subc)) => {
                    match SendAudioOperation::try_from(audio_subc.clone()) {
                        Ok(o) => o.send(),
                        Err(e) => {
                            error!("{}", e.message);
                            process::exit(e.exit_code);
                        }
                    }
                }
                ("document", Some(document_subc)) => {
                    match SendDocumentOperation::try_from(document_subc.clone()) {
                        Ok(o) => o.send(),
                        Err(e) => {
                            error!("{}", e.message);
                            process::exit(e.exit_code);
                        }
                    }
                }
                ("location", Some(location_subc)) => {
                    match SendLocationOperation::try_from(location_subc.clone()) {
                        Ok(o) => o.send(),
                        Err(e) => {
                            error!("{}", e.message);
                            process::exit(e.exit_code);
                        }
                    }
                }
                ("message", Some(message_subc)) => {
                    match SendMessageOperation::try_from(message_subc.clone()) {
                        Ok(o) => o.send(),
                        Err(e) => {
                            error!("{}", e.message);
                            process::exit(e.exit_code);
                        }
                    }
                }
                ("photo", Some(photo_subc)) => {
                    match SendPhotoOperation::try_from(photo_subc.clone()) {
                        Ok(o) => o.send(),
                        Err(e) => {
                            error!("{}", e.message);
                            process::exit(e.exit_code);
                        }
                    }
                }
                ("poll", Some(poll_subc)) => match SendPollOperation::try_from(poll_subc.clone()) {
                    Ok(o) => o.send(),
                    Err(e) => {
                        error!("{}", e.message);
                        process::exit(e.exit_code);
                    }
                },
                ("video", Some(video_subc)) => {
                    match SendVideoOperation::try_from(video_subc.clone()) {
                        Ok(o) => o.send(),
                        Err(e) => {
                            error!("{}", e.message);
                            process::exit(e.exit_code);
                        }
                    }
                }
                (&_, _) => unimplemented!(),
            },
            (&_, _) => unimplemented!(),
        },
        (&_, _) => unimplemented!(),
    }
}
