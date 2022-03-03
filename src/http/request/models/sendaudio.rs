use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;

use crate::operations::{
    bot::send::{self, audio::SendAudioParams},
    OperationError,
};

use super::{ChatId, InputFile, ParseMode};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug)]
pub(crate) struct SendAudioRequestModel {
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
            InputFile::Local(p) => caption_form
                .file("audio", p)
                .map_err(|err| OperationError::IoError(err))?,
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
