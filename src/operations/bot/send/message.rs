use std::convert::TryInto;

use reqwest::blocking::Client;

use crate::{
    http::{
        request::models::sendmessage::SendMessageRequestModel,
        response::models::GenericResponseModel,
    },
    operations::{bot::BotParams, OperationError, RootParams},
    API_ROOT_URL,
};

use super::{SendOperation, SendParams};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug)]
pub(crate) struct MessageParams {
    pub(crate) message: String,
}

impl MessageParams {
    pub(crate) fn new(message: String) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub(crate) type SendMessageParams = (RootParams, BotParams, SendParams, MessageParams);

#[derive(Debug)]
pub(crate) struct SendMessageOperation {
    params: SendMessageParams,
}

impl SendMessageOperation {
    pub(crate) fn new(params: SendMessageParams) -> Self {
        Self { params }
    }
}

impl SendOperation for SendMessageOperation {
    fn send(self) -> Result<(), OperationError> {
        info!("✏️ Sending message...");

        let url = format!(
            "{root_url}{token}/sendMessage",
            root_url = API_ROOT_URL,
            token = self.params.1.token,
        );
        trace!("url: {}", url);

        let req_instance: SendMessageRequestModel = self.params.into();
        let req_body = match req_instance.try_into() {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        trace!("request body: {:?}", req_body);

        // TODO set up client earlier on bot params
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
