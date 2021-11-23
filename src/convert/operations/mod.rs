use std::convert::TryFrom;

use clap::ArgMatches;

use crate::operations::{OperationError, RootParams};

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

pub mod bot;

impl TryFrom<ArgMatches<'static>> for RootParams {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        debug!("Converting ArgMatches to RootParams...");
        trace!("arg matches: {:?}", m);
        let params = RootParams::new(if m.is_present("no-secure") {
            false
        } else {
            true
        });
        trace!("root params: {:?}", params);
        Ok(params)
    }
}
