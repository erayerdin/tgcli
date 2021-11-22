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
pub struct PollParams {
    pub question: String,
    pub options: Vec<String>,
}

impl PollParams {
    pub fn new(question: String, options: Vec<String>) -> Self {
        Self { question, options }
    }
}

pub type SendPollParams = (RootParams, BotParams, SendParams, PollParams);

#[derive(Debug)]
pub struct SendPollOperation {
    params: SendPollParams,
}

impl SendPollOperation {
    pub fn new(params: SendPollParams) -> Self {
        Self { params }
    }
}

impl SendOperation for SendPollOperation {
    fn send(self) -> Result<(), crate::operations::OperationError> {
        todo!() // TODO implement SendPollOperation
    }
}
