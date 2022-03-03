use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;

use crate::operations::{
    bot::send::{self, video::SendVideoParams},
    OperationError,
};

use super::{ChatId, InputFile, ParseMode};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug)]
pub(crate) struct SendVideoRequestModel {
    chat_id: ChatId,
    video: InputFile,
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
            InputFile::Local(p) => caption_form
                .file("video", p)
                .map_err(|err| OperationError::IoError(err))?,
            InputFile::Remote(u) => caption_form.text("video", u.to_string()),
            InputFile::Id(i) => caption_form.text("video", i),
        };

        let notification_form = match m.disable_notification {
            true => video_form.text("disable_notification", "true"),
            false => video_form,
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

        let disable_notification = params.2.silent;

        SendVideoRequestModel {
            chat_id,
            caption,
            parse_mode,
            video,
            disable_notification,
        }
    }
}
