use std::convert::TryFrom;

use clap::ArgMatches;

use crate::operations::{
    bot::{
        send::{
            poll::{PollParams, SendPollOperation},
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

impl TryFrom<ArgMatches<'static>> for PollParams {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        debug!("Converting ArgMatches to PollParams...");
        trace!("arg matches: {:?}", m);

        let params = PollParams::new(
            m.value_of("question").unwrap().to_string(),
            m.values_of("option")
                .unwrap()
                .map(|v| v.to_string())
                .collect(),
        );
        trace!("poll params: {:?}", params);
        Ok(params)
    }
}

impl TryFrom<ArgMatches<'static>> for SendPollOperation {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        debug!("Converting ArgMatches to SendPollOperation...");

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

        let poll_params = match PollParams::try_from(m.clone()) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        Ok(SendPollOperation::new((
            root_params,
            bot_params,
            send_params,
            poll_params,
        )))
    }
}
