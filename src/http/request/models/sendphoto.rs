use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;

use crate::operations::{
    bot::send::{self, photo::SendPhotoParams},
    OperationError,
};

use super::{ChatId, InputFile, ParseMode};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug)]
pub(crate) struct SendPhotoRequestModel {
    chat_id: ChatId,
    photo: InputFile,
    caption: Option<String>,
    parse_mode: ParseMode,
    disable_notification: bool,
    protect_content: bool,
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
            InputFile::Local(p) => caption_form
                .file("photo", p)
                .map_err(|err| OperationError::IoError(err))?,
            InputFile::Remote(u) => caption_form.text("photo", u.to_string()),
            InputFile::Id(i) => caption_form.text("photo", i),
        };

        let notification_form = match m.disable_notification {
            true => photo_form.text("disable_notification", "true"),
            false => photo_form,
        };
        let protect_content_form = match m.protect_content {
            true => notification_form.text("protect_content", "true"),
            false => notification_form,
        };

        Ok(protect_content_form)
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
        let protect_content = params.2.protect_content;

        SendPhotoRequestModel {
            chat_id,
            caption,
            parse_mode,
            photo,
            disable_notification,
            protect_content,
        }
    }
}
