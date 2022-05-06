use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;

use crate::operations::{bot::send::location::SendLocationParams, OperationError};

use super::ChatId;

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug)]
pub(crate) struct SendLocationRequestModel {
    chat_id: ChatId,
    latitude: f32,
    longitude: f32,
    disable_notification: bool,
    protect_content: bool,
}

impl TryFrom<SendLocationRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendLocationRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting SendLocationRequestModel to Form...");
        let chat_id = m.chat_id.to_string();

        let initial_form = Form::new()
            .text("chat_id", chat_id)
            .text("latitude", m.latitude.to_string())
            .text("longitude", m.longitude.to_string());

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

impl From<SendLocationParams> for SendLocationRequestModel {
    fn from(params: SendLocationParams) -> Self {
        debug!("Converting SendLocationParams to SendLocationRequestModel...");

        let chat_id = match params.2.receiver.parse::<usize>() {
            Ok(v) => ChatId::Int(v),
            Err(_) => ChatId::Str(params.2.receiver),
        };

        let latitude = params.3.latitude;
        let longitude = params.3.longitude;
        let disable_notification = params.2.silent;
        let protect_content = params.2.protect_content;

        SendLocationRequestModel {
            chat_id,
            latitude,
            longitude,
            disable_notification,
            protect_content,
        }
    }
}
