use clap::ArgMatches;

use crate::operations::bot::send::{MessageFormat, SendParams};

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

impl From<&str> for MessageFormat {
    fn from(v: &str) -> Self {
        log::debug!("Converting {} to MessageFormat...", v);

        match v {
            "markdown" => MessageFormat::Markdown,
            "html" => MessageFormat::HTML,
            _ => {
                log::warn!("Unknown message format was provided. Falling back to markdown.");
                MessageFormat::Markdown
            }
        }
    }
}

impl From<ArgMatches<'static>> for SendParams {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to SendParams...");
        log::debug!("arg matches: {:?}", m);
        let params = SendParams::new(
            m.value_of("receiver").unwrap(),
            MessageFormat::from(m.value_of("format").unwrap()),
        );
        log::debug!("send params: {:?}", params);
        params
    }
}
