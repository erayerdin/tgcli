use std::convert::TryFrom;

use clap::ArgMatches;

use crate::operations::{
    bot::{
        send::{
            message::{MessageParams, SendMessageOperation},
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

impl TryFrom<ArgMatches<'static>> for MessageParams {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        log::debug!("Converting ArgMatches to MessageParams...");
        log::debug!("arg params: {:?}", m);

        let params = MessageParams::new(m.value_of("message").unwrap().to_owned());
        log::trace!("message params: {:?}", params);
        Ok(params)
    }
}

impl TryFrom<ArgMatches<'static>> for SendMessageOperation {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        log::debug!("Converting ArgMatches to SendMessageOperation...");

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

        let message_params = match MessageParams::try_from(m.clone()) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        Ok(SendMessageOperation::new((
            root_params,
            bot_params,
            send_params,
            message_params,
        )))
    }
}
