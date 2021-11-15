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

impl From<ArgMatches<'static>> for PollParams {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to PollParams...");
        log::trace!("arg matches: {:?}", m);

        let params = PollParams::new(
            m.value_of("question").unwrap().to_string(),
            m.values_of("option")
                .unwrap()
                .map(|v| v.to_string())
                .collect(),
        );
        log::trace!("poll params: {:?}", params);
        params
    }
}

impl From<ArgMatches<'static>> for SendPollOperation {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to SendPollOperation...");

        SendPollOperation::new((
            // TODO implement RootParams error
            RootParams::try_from(m.clone()).expect("This error is to be implemented."),
            // TODO implement this error
            BotParams::try_from(m.clone()).expect("This error is to be implemented."),
            // TODO implement SendParams error
            SendParams::try_from(m.clone()).expect("This error is to be implemented."),
            PollParams::from(m.clone()),
        ))
    }
}
