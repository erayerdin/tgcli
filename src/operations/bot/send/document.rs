use std::{convert::TryInto, path::PathBuf};

use reqwest::blocking::Client;

use crate::{
    handle_response,
    http::request::models::senddocument::SendDocumentRequestModel,
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
pub struct DocumentParams {
    pub file: PathBuf,
    pub thumbnail: Option<PathBuf>,
    pub message: Option<String>,
}

impl DocumentParams {
    pub fn new(file: PathBuf, thumbnail: Option<PathBuf>, message: Option<String>) -> Self {
        Self {
            file,
            thumbnail,
            message,
        }
    }
}

pub type SendDocumentParams = (RootParams, BotParams, SendParams, DocumentParams);

#[derive(Debug)]
pub struct SendDocumentOperation {
    params: SendDocumentParams,
}

impl SendDocumentOperation {
    pub fn new(params: SendDocumentParams) -> Self {
        Self { params }
    }
}

impl SendOperation for SendDocumentOperation {
    fn send(self) -> Result<(), OperationError> {
        info!("Sending document...");

        let url = format!(
            "{root_url}{token}/sendDocument",
            root_url = API_ROOT_URL,
            token = self.params.1.token
        );
        trace!("url: {}", url);

        let req_instance: SendDocumentRequestModel = self.params.into();
        let req_body = match req_instance.try_into() {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        debug!("request body: {:?}", req_body);

        let client = Client::new();
        let response = client.post(url).multipart(req_body).send();

        handle_response!(response, on_success => {
            info!("Successfully sent document.");
        }, on_failure => {
            error!("An error occurred while sending the document.");
        })
    }
}
