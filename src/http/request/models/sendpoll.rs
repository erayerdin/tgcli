use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;
use serde_json::json;

use crate::operations::{bot::send::poll::SendPollParams, OperationError};

use super::ChatId;

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug)]
pub(crate) struct SendPollRequestModel {
    chat_id: ChatId,
    question: String,
    options: Vec<String>,
    disable_notification: bool,
    protect_content: bool,
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

        let protect_content_form = match m.protect_content {
            true => notification_form.text("protect_content", "true"),
            false => notification_form,
        };

        Ok(protect_content_form)
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
        let protect_content = params.2.protect_content;

        SendPollRequestModel {
            chat_id,
            question,
            options,
            disable_notification,
            protect_content,
        }
    }
}
