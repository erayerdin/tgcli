use crate::{
    http::request::models::sendmessage::SendMessageRequestModel,
    operations::{bot::BotParams, CommonExitCodes, OperationError, RootParams},
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
pub struct MessageParams {
    pub message: String,
}

impl MessageParams {
    pub fn new(message: String) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub type SendMessageParams = (RootParams, BotParams, SendParams, MessageParams);

#[derive(Debug)]
pub struct SendMessageOperation {
    params: SendMessageParams,
}

impl SendMessageOperation {
    pub fn new(params: SendMessageParams) -> Self {
        Self { params }
    }
}

impl SendOperation for SendMessageOperation {
    fn send(self) -> Result<(), OperationError> {
        log::info!("Sending message...");

        // TODO use base url as constant
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.params.1.token
        );
        log::trace!("url: {}", url);

        let req_body: SendMessageRequestModel = self.params.into();
        log::trace!("request body: {:?}", req_body);

        // TODO set up client earlier on bot params
        let client = reqwest::blocking::Client::new();
        let response = client.post(url).multipart(req_body.into()).send();

        match response {
            Ok(r) => {
                // TODO model response body
                if r.status().is_success() {
                    log::info!("Successfully sent the message.");
                    log::trace!("response: {:?}", r);
                    Ok(())
                } else {
                    log::error!("A request error occured while sending the message.");
                    Err(OperationError::new(
                        CommonExitCodes::ReqwestHttpError as i32,
                        &format!(
                            "An error occured while sending the message. {}",
                            r.text().unwrap() // TODO implement better error reporting
                        ),
                    ))
                }
            }
            Err(e) => Err(OperationError::new(
                CommonExitCodes::ReqwestConnectionError as i32,
                &format!("An error occured while connecting to Telegram API. {}", e),
            )),
        }
    }
}
