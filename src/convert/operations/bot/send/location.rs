use clap::ArgMatches;

use crate::operations::{
    bot::{
        send::{
            location::{LocationParams, SendLocationOperation},
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

impl From<ArgMatches<'static>> for LocationParams {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to LocationParams...");
        log::trace!("arg matches: {:?}", m);
        let params = LocationParams::new(
            m.value_of("latitude").unwrap().parse().unwrap(),
            m.value_of("longitude").unwrap().parse().unwrap(),
        );
        log::trace!("location params: {:?}", params);
        params
    }
}

impl From<ArgMatches<'static>> for SendLocationOperation {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to SendLocationOperation...");

        SendLocationOperation::new((
            RootParams::from(m.clone()),
            BotParams::from(m.clone()),
            SendParams::from(m.clone()),
            LocationParams::from(m.clone()),
        ))
    }
}
