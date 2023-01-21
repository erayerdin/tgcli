use std::{convert::TryInto, path::PathBuf};

use reqwest::blocking::Client;

use crate::{
    http::{
        request::models::senddocument::SendDocumentRequestModel,
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
pub(crate) struct DocumentParams {
    pub(crate) file: PathBuf,
    pub(crate) thumbnail: Option<PathBuf>,
    pub(crate) message: Option<String>,
}

impl DocumentParams {
    pub(crate) fn new(file: PathBuf, thumbnail: Option<PathBuf>, message: Option<String>) -> Self {
        Self {
            file,
            thumbnail,
            message,
        }
    }
}

pub(crate) type SendDocumentParams = (RootParams, BotParams, SendParams, DocumentParams);

#[derive(Debug)]
pub(crate) struct SendDocumentOperation {
    params: SendDocumentParams,
}

impl SendDocumentOperation {
    pub(crate) fn new(params: SendDocumentParams) -> Self {
        Self { params }
    }
}

impl SendOperation for SendDocumentOperation {
    fn send(self) -> Result<(), OperationError> {
        info!("📎 Sending document...");

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
