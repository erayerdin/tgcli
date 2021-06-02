use std::path::PathBuf;

use crate::operations::{bot::BotParams, RootParams};

use super::{SendOperation, SendParams};

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

#[derive(Debug)]
pub struct DocumentParams {
    file: PathBuf,
    thumbnail: Option<PathBuf>,
    message: Option<String>,
}

impl DocumentParams {
    pub fn new(file: PathBuf, thumbnail: Option<PathBuf>, message: Option<String>) -> Self {
        Self {
            file,
            thumbnail,
            message,
        }
    }
}

#[derive(Debug)]
pub struct SendDocumentOperation {
    params: (RootParams, BotParams, SendParams, DocumentParams),
}

impl SendDocumentOperation {
    pub fn new(params: (RootParams, BotParams, SendParams, DocumentParams)) -> Self {
        Self { params }
    }
}

impl SendOperation for SendDocumentOperation {
    fn send(self) -> Result<(), crate::operations::OperationError> {
        todo!() // TODO implement SendDocumentOperation
    }
}