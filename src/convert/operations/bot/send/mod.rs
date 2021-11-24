use std::convert::TryFrom;

use clap::ArgMatches;

use crate::operations::{
    bot::send::{MessageFormat, SendParams},
    CommonExitCodes, OperationError,
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

pub mod audio;
pub mod document;
pub mod location;
pub mod message;
pub mod photo;
pub mod poll;
pub mod video;

impl From<&str> for MessageFormat {
    fn from(v: &str) -> Self {
        debug!("Converting {} to MessageFormat...", v);

        match v {
            "markdown" => MessageFormat::Markdown,
            "html" => MessageFormat::HTML,
            _ => {
                warn!("Unknown message format was provided. Falling back to markdown.");
                MessageFormat::Markdown
            }
        }
    }
}

impl TryFrom<ArgMatches<'static>> for SendParams {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        debug!("Converting ArgMatches to SendParams...");
        trace!("arg matches: {:?}", m);

        let receiver = match m.value_of("receiver") {
            Some(r) => r,
            None => {
                return Err(OperationError::new(
                    CommonExitCodes::ClapMissingValue as i32,
                    "`receiver` is a required argument on `send` subcommand but is missing.",
                ))
            }
        };

        let format = match m.value_of("format") {
            Some(f) => f,
            None => {
                return Err(OperationError::new(
                    CommonExitCodes::ClapMissingValue as i32,
                    "`format` is a required argument on `send` subcommand but is missing.",
                ))
            }
        };

        let silent = m.is_present("silent");

        let params = SendParams::new(receiver, MessageFormat::from(format), silent);
        trace!("send params: {:?}", params);
        Ok(params)
    }
}
