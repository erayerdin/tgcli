use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;

use crate::operations::{bot::send::location::SendLocationParams, OperationError};

use super::ChatId;

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
pub(crate) struct SendLocationRequestModel {
    chat_id: ChatId,
    latitude: f32,
    longitude: f32,
    disable_notification: bool,
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

        Ok(notification_form)
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

        SendLocationRequestModel {
            chat_id,
            latitude,
            longitude,
            disable_notification,
        }
    }
}
