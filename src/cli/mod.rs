use std::path;

use crate::{
    cli::{
        logging::set_logger,
        validators::{
            audio_validator, caption_validator, file_presence_validator, image_validator,
            poll_option_validator, poll_question_validator, video_validator,
        },
    },
    operations::{
        bot::{
            send::{
                audio::{AudioParams, SendAudioOperation},
                document::{DocumentParams, SendDocumentOperation},
                location::{LocationParams, SendLocationOperation},
                message::{MessageParams, SendMessageOperation},
                photo::{PhotoParams, SendPhotoOperation},
                poll::{PollParams, SendPollOperation},
                video::{SendVideoOperation, VideoParams},
                MessageFormat, SendOperation, SendParams,
            },
            BotParams,
        },
        OperationError, RootParams,
    },
};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod logging;
pub(crate) mod validators;

#[derive(Parser)]
pub struct Cli {
    /// How much verbose the output should be.
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[clap(subcommand)]
    subcommands: CliSubcommands,
}

#[derive(Subcommand)]
enum CliSubcommands {
    /// The operation for bots.
    Bot {
        /// The token of bot.
        #[clap(short, long, env = "TELEGRAM_BOT_TOKEN")]
        token: String,
        #[clap(subcommand)]
        subcommands: BotSubcommands,
    },
}

#[derive(Subcommand)]
enum BotSubcommands {
    /// Send various types of messages.
    Send {
        /// The user id of receiver.
        #[clap(short, long)]
        receiver: String,
        /// The format of message.
        #[clap(long, value_enum, default_value = "markdown")]
        format: MessageFormat,
        /// Whether to send notification to user.
        #[clap(long)]
        silent: bool,
        /// Do not let user to forward or save message.
        #[clap(long)]
        protect_content: bool,
        #[clap(subcommand)]
        subcommands: SendSubcommands,
    },
}

#[derive(Subcommand)]
pub(crate) enum SendSubcommands {
    /// A regular text message.
    Message { message: String },
    /// A file.
    Document {
        /// The file to be sent.
        #[clap(value_parser = file_presence_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long, value_parser = caption_validator)]
        message: Option<String>,
        /// A preview image for file.
        #[clap(long, value_parser = file_presence_validator)]
        thumbnail: Option<path::PathBuf>,
    },
    /// A message with image viewer.
    Photo {
        /// The file to be sent.
        #[clap(value_parser = image_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long, value_parser = caption_validator)]
        message: Option<String>,
    },
    /// A message with video viewer.
    Video {
        /// The file to be sent.
        #[clap(value_parser = video_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long, value_parser = caption_validator)]
        message: Option<String>,
    },
    /// A message with audio player.
    Audio {
        /// The file to be sent.
        #[clap(value_parser = audio_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long, value_parser = caption_validator)]
        message: Option<String>,
        #[clap(long)]
        performer: Option<String>,
        #[clap(long)]
        title: Option<String>,
    },
    /// A message with question and options.
    Poll {
        /// Question.
        #[clap(value_parser = poll_question_validator)]
        question: String,
        /// One of options.
        // max 10, ref: https://core.telegram.org/bots/api#sendpoll
        #[clap(short = 'o', long = "option", num_args = 2..10, value_parser = poll_option_validator)]
        options: Vec<String>,
    },
    /// A location on earth.
    Location {
        /// The latitude of location.
        #[clap(short = 'x', long)]
        latitude: f32,
        /// The longitude of location.
        #[clap(short = 'y', long)]
        longitude: f32,
    },
}

pub fn match_app(cli: Cli) -> Result<(), OperationError> {
    set_logger(&cli.verbose)
        .map_err(|set_logger_error| OperationError::LoggerError(set_logger_error))?;

    let root_params = RootParams::new(cli.verbose);

    match cli.subcommands {
        CliSubcommands::Bot { token, subcommands } => {
            let bot_params = BotParams::new(token);

            match subcommands {
                BotSubcommands::Send {
                    receiver,
                    format,
                    silent,
                    protect_content,
                    subcommands,
                } => {
                    let send_params = SendParams::new(receiver, format, silent, protect_content);

                    match subcommands {
                        SendSubcommands::Message { message } => {
                            let message_params = MessageParams::new(message);
                            let send_message_operation = SendMessageOperation::new((
                                root_params,
                                bot_params,
                                send_params,
                                message_params,
                            ));
                            send_message_operation.send()
                        }
                        SendSubcommands::Document {
                            file,
                            message,
                            thumbnail,
                        } => {
                            let document_params = DocumentParams::new(file, thumbnail, message);
                            let send_document_operation = SendDocumentOperation::new((
                                root_params,
                                bot_params,
                                send_params,
                                document_params,
                            ));
                            send_document_operation.send()
                        }
                        SendSubcommands::Photo { file, message } => {
                            let photo_params = PhotoParams::new(file, message);
                            let send_photo_operation = SendPhotoOperation::new((
                                root_params,
                                bot_params,
                                send_params,
                                photo_params,
                            ));
                            send_photo_operation.send()
                        }
                        SendSubcommands::Video { file, message } => {
                            let video_params = VideoParams::new(file, message);
                            let send_video_operation = SendVideoOperation::new((
                                root_params,
                                bot_params,
                                send_params,
                                video_params,
                            ));
                            send_video_operation.send()
                        }
                        SendSubcommands::Audio {
                            file,
                            message,
                            performer,
                            title,
                        } => {
                            let audio_params = AudioParams::new(file, message, title, performer);
                            let send_audio_operation = SendAudioOperation::new((
                                root_params,
                                bot_params,
                                send_params,
                                audio_params,
                            ));
                            send_audio_operation.send()
                        }
                        SendSubcommands::Poll { question, options } => {
                            let poll_params = PollParams::new(question, options);
                            let send_poll_operation = SendPollOperation::new((
                                root_params,
                                bot_params,
                                send_params,
                                poll_params,
                            ));
                            send_poll_operation.send()
                        }
                        SendSubcommands::Location {
                            latitude,
                            longitude,
                        } => {
                            let location_params = LocationParams::new(latitude, longitude);
                            let send_location_operation = SendLocationOperation::new((
                                root_params,
                                bot_params,
                                send_params,
                                location_params,
                            ));
                            send_location_operation.send()
                        }
                    }
                }
            }
        }
    }
}
