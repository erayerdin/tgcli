use clap::ArgMatches;

use crate::operations::bot::send::location::{LocationParams, SendLocationOperation};

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
        log::debug!("arg matches: {:?}", m);
        let params = LocationParams::new(m.value_of("latitude").unwrap(), m.value_of("longitude").unwrap());
        log::debug!("location params: {:?}", params)
        params
    }
}

impl From<ArgMatches<'static>> for SendLocationOperation {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to SendLocationOperation...");
        log::debug!("arg matches: {:?}");

        SendLocationOperation::new(LocationParams::from(m))
    }
}
