use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;
use serde_json::json;

use crate::operations::OperationError;

use super::ChatId;

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
pub struct SendPollRequestModel {
    chat_id: ChatId,
    question: String,
    options: Vec<String>,
}

impl TryFrom<SendPollRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendPollRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting SendPollRequestModel to Form...");
        let chat_id = m.chat_id.to_string();
        let options = json!(m.options).to_string();

        Ok(Form::new()
            .text("chat_id", chat_id)
            .text("question", m.question)
            .text("options", options))
    }
}
