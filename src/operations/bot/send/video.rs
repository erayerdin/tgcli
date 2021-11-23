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
pub struct VideoParams {
    pub file: PathBuf,
    pub message: Option<String>,
    pub horizontal: Option<usize>,
    pub vertical: Option<usize>,
}

impl VideoParams {
    pub fn new(
        file: PathBuf,
        message: Option<String>,
        horizontal: Option<usize>,
        vertical: Option<usize>,
    ) -> Self {
        Self {
            file,
            message,
            horizontal,
            vertical,
        }
    }
}

pub type SendVideoParams = (RootParams, BotParams, SendParams, VideoParams);

#[derive(Debug)]
pub struct SendVideoOperation {
    params: SendVideoParams,
}

impl SendVideoOperation {
    pub fn new(params: SendVideoParams) -> Self {
        Self { params }
    }
}

impl SendOperation for SendVideoOperation {
    fn send(self) -> Result<(), crate::operations::OperationError> {
        todo!() // TODO implement SendVideoOperation
    }
}
