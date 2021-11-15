use std::{convert::TryFrom, path::PathBuf};

use clap::ArgMatches;

use crate::operations::{
    bot::{
        send::{
            video::{SendVideoOperation, VideoParams},
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

impl From<ArgMatches<'static>> for VideoParams {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to VideoParams...");
        log::trace!("arg matches: {:?}", m);

        let params = VideoParams::new(
            PathBuf::from(m.value_of("file").unwrap()),
            m.value_of("message").map_or(None, |v| Some(v.to_string())),
            m.value_of("horizontal")
                .map_or(None, |v| Some(v.parse().unwrap())),
            m.value_of("vertical")
                .map_or(None, |v| Some(v.parse().unwrap())),
        );
        log::trace!("video params: {:?}", params);
        params
    }
}

impl From<ArgMatches<'static>> for SendVideoOperation {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to SendVideoOperation...");

        SendVideoOperation::new((
            // TODO implement RootParams error
            RootParams::try_from(m.clone()).expect("This error is to be implemented."),
            // TODO implement this error
            BotParams::try_from(m.clone()).expect("This error is to be implemented."),
            // TODO implement SendParams error
            SendParams::try_from(m.clone()).expect("This error is to be implemented."),
            VideoParams::from(m.clone()),
        ))
    }
}
