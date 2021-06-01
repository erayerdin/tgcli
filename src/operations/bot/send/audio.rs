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

pub struct AudioParams {
    file: PathBuf,
    message: Option<String>,
    title: Option<String>,
    performer: Option<String>,
}

impl AudioParams {
    pub fn new(
        file: PathBuf,
        message: Option<String>,
        title: Option<String>,
        performer: Option<String>,
    ) -> Self {
        Self {
            file,
            message,
            title,
            performer,
        }
    }
}

pub struct SendAudioOperation {
    params: (RootParams, BotParams, SendParams, AudioParams),
}

impl SendAudioOperation {
    pub fn new(params: (RootParams, BotParams, SendParams, AudioParams)) -> Self {
        Self { params }
    }
}

impl SendOperation for SendAudioOperation {
    fn send(self) -> Result<(), crate::operations::OperationError> {
        todo!() // TODO implement SendAudioOperation
    }
}
