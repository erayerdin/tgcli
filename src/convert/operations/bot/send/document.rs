use std::{convert::TryFrom, path::PathBuf};

use clap::ArgMatches;

use crate::operations::{
    bot::{
        send::{
            document::{DocumentParams, SendDocumentOperation},
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

impl TryFrom<ArgMatches<'static>> for DocumentParams {
    type Error = OperationError;

    fn try_from(m: ArgMatches<'static>) -> Result<Self, Self::Error> {
        log::debug!("Converting ArgMatches to DocumentParams...");
        log::trace!("arg matches: {:?}", m);
        let params = DocumentParams::new(
            PathBuf::from(m.value_of("file").unwrap()),
            m.value_of("thumbnail")
                .map_or(None, |v| Some(PathBuf::from(v))),
            m.value_of("message").map_or(None, |v| Some(v.to_string())),
        );
        log::trace!("document params: {:?}", params);
        Ok(params)
    }
}

impl From<ArgMatches<'static>> for SendDocumentOperation {
    fn from(m: ArgMatches<'static>) -> Self {
        log::debug!("Converting ArgMatches to SendDocumentOperation...");

        SendDocumentOperation::new((
            // TODO implement RootParams error
            RootParams::try_from(m.clone()).expect("This error is to be implemented."),
            // TODO implement this error
            BotParams::try_from(m.clone()).expect("This error is to be implemented."),
            // TODO implement SendParams error
            SendParams::try_from(m.clone()).expect("This error is to be implemented."),
            // TODO implement DocumentParams error
            DocumentParams::try_from(m.clone()).expect("This error is to be implemented."),
        ))
    }
}
