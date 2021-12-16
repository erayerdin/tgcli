use std::{convert::TryInto, path::PathBuf};

use reqwest::Client;

use crate::{
    handle_response,
    http::request::models::sendaudio::SendAudioRequestModel,
    operations::{bot::BotParams, RootParams},
    API_ROOT_URL,
};

use super::{SendOperation, SendParams};

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
pub struct AudioParams {
    pub file: PathBuf,
    pub message: Option<String>,
    pub title: Option<String>,
    pub performer: Option<String>,
}

impl AudioParams {
    pub fn new(
        file: PathBuf,
        message: Option<String>,
        title: Option<String>,
        performer: Option<String>,
    ) -> Self {
        Self {
            file,
            message,
            title,
            performer,
        }
    }
}

pub type SendAudioParams = (RootParams, BotParams, SendParams, AudioParams);

#[derive(Debug)]
pub struct SendAudioOperation {
    params: SendAudioParams,
}

impl SendAudioOperation {
    pub fn new(params: SendAudioParams) -> Self {
        Self { params }
    }
}

#[async_trait]
impl SendOperation for SendAudioOperation {
    fn send(self) -> Result<(), crate::operations::OperationError> {
        info!("🔊 Sending audio...");

        let url = format!(
            "{root_url}{token}/sendAudio",
            root_url = API_ROOT_URL,
            token = self.params.1.token
        );
        trace!("url: {}", url);

        let req_instance: SendAudioRequestModel = self.params.into();
        let req_body = match req_instance.try_into() {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        debug!("request body: {:?}", req_body);

        let client = Client::new();
        let response = client.post(url).multipart(req_body).send();

        handle_response!(response, on_success => {
            info!("📦 Successfully sent audio.");
        }, on_failure => {
            error!("☠️ An error occurred while sending the audio.");
        })
    }
}
