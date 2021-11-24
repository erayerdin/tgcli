use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;
use serde_json::json;

use crate::operations::{bot::send::poll::SendPollParams, OperationError};

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
    disable_notification: bool,
}

impl TryFrom<SendPollRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendPollRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting SendPollRequestModel to Form...");
        let chat_id = m.chat_id.to_string();
        let options = json!(m.options).to_string();

        let initial_form = Form::new()
            .text("chat_id", chat_id)
            .text("question", m.question)
            .text("options", options);

        let notification_form = match m.disable_notification {
            true => initial_form.text("disable_notification", "true"),
            false => initial_form,
        };

        Ok(notification_form)
    }
}

impl From<SendPollParams> for SendPollRequestModel {
    fn from(params: SendPollParams) -> Self {
        debug!("Converting SendPollParams to SendPollRequestModel...");

        let chat_id = match params.2.receiver.parse::<usize>() {
            Ok(v) => ChatId::Int(v),
            Err(_) => ChatId::Str(params.2.receiver),
        };

        let question = params.3.question;
        let options = params.3.options;
        let disable_notification = params.2.silent;

        SendPollRequestModel {
            chat_id,
            question,
            options,
            disable_notification,
        }
    }
}
