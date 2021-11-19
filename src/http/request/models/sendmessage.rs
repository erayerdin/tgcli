use reqwest::blocking::multipart::Form;

use crate::operations::bot::send::{self, message::SendMessageParams};

use super::{ChatId, ParseMode};

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
/// A model for /sendMessage request.
pub struct SendMessageRequestModel {
    chat_id: ChatId,
    text: String,
    parse_mode: ParseMode,
}

// TODO change to TryFrom
impl From<SendMessageRequestModel> for Form {
    fn from(m: SendMessageRequestModel) -> Self {
        debug!("Converting SendMessageRequestModel to Form...");
        let chat_id = m.chat_id.to_string();
        let parse_mode = m.parse_mode.to_string();

        Form::new()
            .text("chat_id", chat_id)
            .text("text", m.text)
            .text("parse_mode", parse_mode)
    }
}

impl From<SendMessageParams> for SendMessageRequestModel {
    fn from(params: SendMessageParams) -> Self {
        debug!("Converting SendMessageParams to SendMessageRequestModel...");

        let chat_id = match params.2.receiver.parse::<usize>() {
            Ok(v) => ChatId::Int(v),
            Err(_) => ChatId::Str(params.2.receiver),
        };

        let text = params.3.message;

        let parse_mode = match params.2.format {
            send::MessageFormat::Markdown => ParseMode::Markdown,
            send::MessageFormat::HTML => ParseMode::HTML,
        };

        SendMessageRequestModel {
            chat_id,
            text,
            parse_mode,
        }
    }
}
