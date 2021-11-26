use std::convert::TryFrom;

use futures::executor;
use reqwest::multipart::Form;

use crate::{
    http::request::models::generate_form_part_from_file,
    operations::{
        bot::send::{self, audio::SendAudioParams},
        OperationError,
    },
};

use super::{ChatId, InputFile, ParseMode};

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
pub struct SendAudioRequestModel {
    chat_id: ChatId,
    audio: InputFile,
    performer: Option<String>,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: ParseMode,
    disable_notification: bool,
}

impl TryFrom<SendAudioRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendAudioRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting SendAudioRequestModel to Form...");

        let chat_id = m.chat_id.to_string();
        let parse_mode = m.parse_mode.to_string();

        let initial_form = Form::new()
            .text("chat_id", chat_id)
            .text("parse_mode", parse_mode);

        let caption_form = match m.caption {
            Some(c) => initial_form.text("caption", c),
            None => initial_form,
        };

        let audio_form = match m.audio {
            InputFile::Local(p) => match executor::block_on(generate_form_part_from_file(p)) {
                Ok(part) => caption_form.part("audio", part),
                Err(e) => return Err(e),
            },
            InputFile::Remote(u) => caption_form.text("audio", u.to_string()),
            InputFile::Id(i) => caption_form.text("audio", i),
        };

        let performer_form = match m.performer {
            Some(p) => audio_form.text("performer", p),
            None => audio_form,
        };

        let title_form = match m.title {
            Some(t) => performer_form.text("title", t),
            None => performer_form,
        };

        let notification_form = match m.disable_notification {
            true => title_form.text("disable_notification", "true"),
            false => title_form,
        };

        Ok(notification_form)
    }
}

impl From<SendAudioParams> for SendAudioRequestModel {
    fn from(params: SendAudioParams) -> Self {
        debug!("Converting SendAudioParams to SendAudioRequestModel...");

        let chat_id = match params.2.receiver.parse::<usize>() {
            Ok(v) => ChatId::Int(v),
            Err(_) => ChatId::Str(params.2.receiver),
        };

        let caption = params.3.message;

        let parse_mode = match params.2.format {
            send::MessageFormat::Markdown => ParseMode::Markdown,
            send::MessageFormat::HTML => ParseMode::HTML,
        };

        let audio = InputFile::Local(params.3.file);
        let performer = params.3.performer;
        let title = params.3.title;
        let disable_notification = params.2.silent;

        SendAudioRequestModel {
            chat_id,
            caption,
            parse_mode,
            audio,
            performer,
            title,
            disable_notification,
        }
    }
}
