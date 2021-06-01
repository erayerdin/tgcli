use std::path::PathBuf;

use clap::ArgMatches;

use crate::operations::{
    bot::{
        send::{
            audio::{AudioParams, SendAudioOperation},
            SendParams,
        },
        BotParams,
    },
    RootParams,
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

impl From<ArgMatches<'static>> for AudioParams {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to AudioParams...");
        log::debug!("arg matches: {:?}", m);
        let params = AudioParams::new(
            PathBuf::from(m.value_of("file").unwrap()),
            m.value_of("message").map_or(None, |v| Some(v.to_string())),
            m.value_of("title").map_or(None, |v| Some(v.to_string())),
            m.value_of("performer")
                .map_or(None, |v| Some(v.to_string())),
        );
        log::debug!("audio params: {:?}", params);
        params
    }
}

impl From<ArgMatches<'static>> for SendAudioOperation {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches into SendAudioOperation...");
        log::debug!("arg matches: {:?}", m);

        SendAudioOperation::new((
            RootParams::from(m.clone()),
            BotParams::from(m.clone()),
            SendParams::from(m.clone()),
            AudioParams::from(m.clone()),
        ))
    }
}
