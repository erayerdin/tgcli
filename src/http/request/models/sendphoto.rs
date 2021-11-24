use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;

use crate::operations::{
    bot::send::{self, photo::SendPhotoParams},
    CommonExitCodes, OperationError,
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
pub struct SendPhotoRequestModel {
    chat_id: ChatId,
    photo: InputFile,
    caption: Option<String>,
    parse_mode: ParseMode,
    disable_notification: bool,
}

impl TryFrom<SendPhotoRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendPhotoRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting SendPhotoRequestModel to Form...");
        let chat_id = m.chat_id.to_string();
        let parse_mode = m.parse_mode.to_string();

        let initial_form = Form::new()
            .text("chat_id", chat_id)
            .text("parse_mode", parse_mode);

        let caption_form = match m.caption {
            Some(c) => initial_form.text("caption", c),
            None => initial_form,
        };

        let photo_form = match m.photo {
            InputFile::Local(p) => match caption_form.file("photo", p) {
                Ok(f) => f,
                Err(e) => {
                    return Err(OperationError::new(
                        CommonExitCodes::ReqwestFormError as i32,
                        &format!("Could not send photo to Telegram. {}", e),
                    ))
                }
            },
            InputFile::Remote(u) => caption_form.text("photo", u.to_string()),
            InputFile::Id(i) => caption_form.text("photo", i),
        };

        let notification_form = match m.disable_notification {
            true => photo_form.text("disable_notification", "true"),
            false => photo_form,
        };

        Ok(notification_form)
    }
}

impl From<SendPhotoParams> for SendPhotoRequestModel {
    fn from(params: SendPhotoParams) -> Self {
        debug!("Converting SendPhotoParams to SendPhotoRequestModel...");

        let chat_id = match params.2.receiver.parse::<usize>() {
            Ok(v) => ChatId::Int(v),
            Err(_) => ChatId::Str(params.2.receiver),
        };

        let caption = params.3.message;

        let parse_mode = match params.2.format {
            send::MessageFormat::Markdown => ParseMode::Markdown,
            send::MessageFormat::HTML => ParseMode::HTML,
        };

        let photo = InputFile::Local(params.3.file);

        let disable_notification = params.2.silent;

        SendPhotoRequestModel {
            chat_id,
            caption,
            parse_mode,
            photo,
            disable_notification,
        }
    }
}
