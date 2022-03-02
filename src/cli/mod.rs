use std::path;

use crate::{
    cli::{logging::set_logger, validators::pathbuf_validator},
    operations::{bot::send::MessageFormat, OperationError},
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
    #[clap(short, long = "verbose", parse(from_occurrences))]
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
        #[clap(validator = pathbuf_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long)]
        message: Option<String>,
        /// A preview image for file.
        #[clap(long, validator = pathbuf_validator)]
        thumbnail: Option<path::PathBuf>,
    },
    /// A message with image viewer.
    Photo {
        /// The file to be sent.
        #[clap(validator = pathbuf_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long)]
        message: Option<String>,
    },
    /// A message with video viewer.
    Video {
        /// The file to be sent.
        #[clap(validator = pathbuf_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long)]
        message: Option<String>,
    },
    /// A message with audio player.
    Audio {
        /// The file to be sent.
        #[clap(validator = pathbuf_validator)]
        file: path::PathBuf,
        /// The file caption.
        #[clap(short, long)]
        message: Option<String>,
        #[clap(long)]
        performer: Option<String>,
        #[clap(long)]
        title: Option<String>,
    },
    /// A message with question and options.
    Poll {
        /// Question.
        question: String,
        /// One of options.
        #[clap(short = 'o', long = "option", multiple_occurrences = true)]
        options: Vec<String>,
    },
}

pub fn match_app(cli: Cli) -> Result<(), OperationError> {
    set_logger(cli.verbosity)
        .map_err(|set_logger_error| OperationError::LoggerError(set_logger_error))?;

    todo!()
}
