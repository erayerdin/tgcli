use std::path;

use crate::{
    cli::{
        logging::set_logger,
        validators::{
            caption_validator, file_presence_validator, poll_option_validator,
            poll_question_validator,
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

pub(crate) mod logging;
pub(crate) mod validators;

#[derive(Parser)]
pub struct Cli {
    /// How much verbose the output should be.
    #[clap(short, long = "verbose", max_occurrences = 4, parse(from_occurrences))]
    verbosity: u8,
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
        #[clap(long, arg_enum, default_value = "markdown")]
        format: MessageFormat,
        /// Whether to send notification to user.
        #[clap(long)]
        silent: bool,
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
        #[clap(validator = file_presence_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long, validator = caption_validator)]
        message: Option<String>,
        /// A preview image for file.
        #[clap(long, validator = file_presence_validator)]
        thumbnail: Option<path::PathBuf>,
    },
    /// A message with image viewer.
    Photo {
        /// The file to be sent.
        #[clap(validator = file_presence_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long, validator = caption_validator)]
        message: Option<String>,
    },
    /// A message with video viewer.
    Video {
        /// The file to be sent.
        #[clap(validator = file_presence_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long, validator = caption_validator)]
        message: Option<String>,
    },
    /// A message with audio player.
    Audio {
        /// The file to be sent.
        #[clap(validator = file_presence_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long, validator = caption_validator)]
        message: Option<String>,
        #[clap(long)]
        performer: Option<String>,
        #[clap(long)]
        title: Option<String>,
    },
    /// A message with question and options.
    Poll {
        /// Question.
        #[clap(validator = poll_question_validator)]
        question: String,
        /// One of options.
        #[clap(short = 'o', long = "option", multiple_occurrences = true, min_values = 2, validator = poll_option_validator)]
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
    set_logger(cli.verbosity)
        .map_err(|set_logger_error| OperationError::LoggerError(set_logger_error))?;

    let root_params = RootParams::new(cli.verbosity);

    match cli.subcommands {
        CliSubcommands::Bot { token, subcommands } => {
            let bot_params = BotParams::new(token);

            match subcommands {
                BotSubcommands::Send {
                    receiver,
                    format,
                    silent,
                    subcommands,
                } => {
                    let send_params = SendParams::new(receiver, format, silent);

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
