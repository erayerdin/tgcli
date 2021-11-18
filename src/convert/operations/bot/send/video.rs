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
    OperationError, RootParams,
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

impl TryFrom<ArgMatches<'static>> for VideoParams {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        debug!("Converting ArgMatches to VideoParams...");
        trace!("arg matches: {:?}", m);

        let params = VideoParams::new(
            PathBuf::from(m.value_of("file").unwrap()),
            m.value_of("message").map_or(None, |v| Some(v.to_string())),
            m.value_of("horizontal")
                .map_or(None, |v| Some(v.parse().unwrap())),
            m.value_of("vertical")
                .map_or(None, |v| Some(v.parse().unwrap())),
        );
        trace!("video params: {:?}", params);
        Ok(params)
    }
}

impl TryFrom<ArgMatches<'static>> for SendVideoOperation {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        debug!("Converting ArgMatches to SendVideoOperation...");

        let root_params = match RootParams::try_from(m.clone()) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        let bot_params = match BotParams::try_from(m.clone()) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        let send_params = match SendParams::try_from(m.clone()) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        let video_params = match VideoParams::try_from(m.clone()) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        Ok(SendVideoOperation::new((
            root_params,
            bot_params,
            send_params,
            video_params,
        )))
    }
}
