use std::string;

use reqwest::blocking::multipart::Form;

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

/// What the type of ChatId is.
enum ChatId {
    Int(usize),
    Str(String),
}

impl string::ToString for ChatId {
    fn to_string(&self) -> String {
        match self {
            ChatId::Int(v) => v.to_string(),
            ChatId::Str(v) => v.clone(),
        }
    }
}

/// Which format Telegram should handle the message text in.
enum ParseMode {
    Markdown2,
    Markdown,
    HTML,
}

impl string::ToString for ParseMode {
    fn to_string(&self) -> String {
        match self {
            ParseMode::Markdown2 => "MarkdownV2".to_owned(),
            ParseMode::Markdown => "Markdown".to_owned(),
            ParseMode::HTML => "HTML".to_owned(),
        }
    }
}

/// A model for /sendMessage request.
struct SendMessageRequestModel {
    chat_id: ChatId,
    text: String,
    parse_mode: ParseMode,
}

impl From<SendMessageRequestModel> for Form {
    fn from(m: SendMessageRequestModel) -> Self {
        let chat_id = m.chat_id.to_string();
        let parse_mode = m.parse_mode.to_string();

        Form::new()
            .text("chat_id", chat_id)
            .text("text", m.text)
            .text("parse_mode", parse_mode)
    }
}
