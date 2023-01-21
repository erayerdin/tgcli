use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;

use crate::operations::{
    bot::send::{self, message::SendMessageParams},
    OperationError,
};

use super::{ChatId, ParseMode};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug)]
/// A model for /sendMessage request.
pub(crate) struct SendMessageRequestModel {
    chat_id: ChatId,
    text: String,
    parse_mode: ParseMode,
    disable_notification: bool,
    protect_content: bool,
}

impl TryFrom<SendMessageRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendMessageRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting SendMessageRequestModel to Form...");
        let chat_id = m.chat_id.to_string();
        let parse_mode = m.parse_mode.to_string();

        let initial_form = Form::new()
            .text("chat_id", chat_id)
            .text("text", m.text)
            .text("parse_mode", parse_mode);

        let notification_form = match m.disable_notification {
            true => initial_form.text("disable_notification", "true"),
            false => initial_form,
        };

        let protect_content_form = match m.protect_content {
            true => notification_form.text("protect_content", "true"),
            false => notification_form,
        };

        Ok(protect_content_form)
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

        let disable_notification = params.2.silent;
        let protect_content = params.2.protect_content;

        SendMessageRequestModel {
            chat_id,
            text,
            parse_mode,
            disable_notification,
            protect_content,
        }
    }
}
