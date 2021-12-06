use std::convert::TryFrom;

use clap::ArgMatches;

use crate::operations::{
    bot::{
        send::{
            location::{LocationParams, SendLocationOperation},
            SendParams,
        },
        BotParams,
    },
    CommonExitCodes, OperationError, RootParams,
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

impl TryFrom<ArgMatches<'static>> for LocationParams {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        debug!("Converting ArgMatches to LocationParams...");
        trace!("arg matches: {:?}", m);

        let latitude: f32 =
            match m.value_of("latitude") {
                Some(l) => match l.parse() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(OperationError::new(
                            CommonExitCodes::StdInvalidValue as i32,
                            "`latitude` argument must be a valid 32-bit floating point number.",
                            None::<&str>,
                        ))
                    }
                },
                None => return Err(OperationError::new(
                    CommonExitCodes::ClapMissingValue as i32,
                    "`latitude` is a required argument on `location` subcommand but is missing.",
                    None::<&str>,
                )),
            };

        let longitude: f32 =
            match m.value_of("longitude") {
                Some(l) => {
                    match l.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(OperationError::new(
                            CommonExitCodes::StdInvalidValue as i32,
                            "`longitude` argument must be a valid 32-bit floating point number.",
                            None::<&str>,
                        )),
                    }
                }
                None => return Err(OperationError::new(
                    CommonExitCodes::ClapMissingValue as i32,
                    "`longitude` is a required argument on `location` subcommand but is missing.",
                    None::<&str>,
                )),
            };

        let params = LocationParams::new(latitude, longitude);
        trace!("location params: {:?}", params);
        Ok(params)
    }
}

impl TryFrom<ArgMatches<'static>> for SendLocationOperation {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        debug!("Converting ArgMatches to SendLocationOperation...");

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

        let location_params = match LocationParams::try_from(m.clone()) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        Ok(SendLocationOperation::new((
            root_params,
            bot_params,
            send_params,
            location_params,
        )))
    }
}
