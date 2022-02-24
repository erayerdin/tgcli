use std::{convert::TryInto, path::PathBuf};

use reqwest::blocking::Client;

use crate::{
    http::{
        request::models::sendaudio::SendAudioRequestModel, response::models::GenericResponseModel,
    },
    operations::{bot::BotParams, OperationError, RootParams},
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
pub(crate) struct AudioParams {
    pub(crate) file: PathBuf,
    pub(crate) message: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) performer: Option<String>,
}

impl AudioParams {
    pub(crate) fn new(
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

pub(crate) type SendAudioParams = (RootParams, BotParams, SendParams, AudioParams);

#[derive(Debug)]
pub(crate) struct SendAudioOperation {
    params: SendAudioParams,
}

impl SendAudioOperation {
    pub(crate) fn new(params: SendAudioParams) -> Self {
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

        // TODO add logging to response and data
        let client = Client::new();
        let response = client
            .post(url)
            .multipart(req_body)
            .send()
            .map_err(|err| OperationError::ReqwestError(err))?;
        let data = response
            .json::<GenericResponseModel>()
            .map_err(|err| OperationError::ReqwestError(err))?;

        if let Some(description) = data.description {
            return Err(OperationError::TelegramAPIError { description });
        }

        Ok(())
    }
}
