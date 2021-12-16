use std::convert::TryFrom;

use futures::executor;
use reqwest::blocking::multipart::Form;

use crate::{
    http::request::models::generate_form_part_from_file,
    operations::{
        bot::send::{self, video::SendVideoParams},
        CommonExitCodes, OperationError,
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
pub struct SendVideoRequestModel {
    chat_id: ChatId,
    video: InputFile,
    width: Option<usize>,
    height: Option<usize>,
    caption: Option<String>,
    parse_mode: ParseMode,
    disable_notification: bool,
}

impl TryFrom<SendVideoRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendVideoRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting to SendVideoRequestModel to Form...");
        let chat_id = m.chat_id.to_string();
        let parse_mode = m.parse_mode.to_string();

        let initial_form = Form::new()
            .text("chat_id", chat_id)
            .text("parse_mode", parse_mode);

        let caption_form = match m.caption {
            Some(c) => initial_form.text("caption", c),
            None => initial_form,
        };

        let video_form = match m.video {
            InputFile::Local(p) => match caption_form.file("video", p) {
                Ok(file) => file,
                Err(e) => {
                    return Err(OperationError::new(
                        CommonExitCodes::ReqwestFormError as i32,
                        "An error occured while attaching file to request form.",
                        Some(e),
                    ))
                }
            },
            InputFile::Remote(u) => caption_form.text("video", u.to_string()),
            InputFile::Id(i) => caption_form.text("video", i),
        };

        let width_form = match m.width {
            Some(w) => video_form.text("width", w.to_string()),
            None => video_form,
        };

        let height_form = match m.height {
            Some(h) => width_form.text("height", h.to_string()),
            None => width_form,
        };

        let notification_form = match m.disable_notification {
            true => height_form.text("disable_notification", "true"),
            false => height_form,
        };

        Ok(notification_form)
    }
}

impl From<SendVideoParams> for SendVideoRequestModel {
    fn from(params: SendVideoParams) -> Self {
        debug!("Converting SendVideoParams to SendVideoRequestModel...");

        let chat_id = match params.2.receiver.parse::<usize>() {
            Ok(v) => ChatId::Int(v),
            Err(_) => ChatId::Str(params.2.receiver),
        };

        let caption = params.3.message;

        let parse_mode = match params.2.format {
            send::MessageFormat::Markdown => ParseMode::Markdown,
            send::MessageFormat::HTML => ParseMode::HTML,
        };

        let video = InputFile::Local(params.3.file);
        let width = params.3.horizontal;
        let height = params.3.vertical;

        let disable_notification = params.2.silent;

        SendVideoRequestModel {
            chat_id,
            caption,
            parse_mode,
            video,
            width,
            height,
            disable_notification,
        }
    }
}
