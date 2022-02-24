use std::{error, fmt};

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

pub(crate) mod bot;

#[derive(Debug)]
pub(crate) struct RootParams;

impl RootParams {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub enum OperationError {
    IoError(std::io::Error),
    ParseFloatError(std::num::ParseFloatError),
    LoggerError(log::SetLoggerError),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    MissingArgument { subc_name: String, arg_name: String },
    TelegramAPIError { description: String },
}

impl error::Error for OperationError {}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationError::IoError(err) => err.fmt(f),
            OperationError::ParseFloatError(err) => err.fmt(f),
            OperationError::LoggerError(err) => err.fmt(f),
            OperationError::ReqwestError(err) => err.fmt(f),
            OperationError::MissingArgument {
                subc_name,
                arg_name,
            } => write!(
                f,
                "`{}` is a required argument for `{}` subcommand but is missing.",
                arg_name, subc_name
            ),
            OperationError::TelegramAPIError { description } => {
                write!(f, "An error occured on Telegram API: {}", description)
            }
            OperationError::SerdeJsonError(err) => err.fmt(f),
        }
    }
}

impl OperationError {
    pub fn exit_code(&self) -> i32 {
        // should we cover exit code for each error kind?
        match self {
            OperationError::IoError(_) => 100,
            OperationError::ParseFloatError(_) => 200,
            OperationError::LoggerError(_) => 300,
            OperationError::ReqwestError(_) => 400,
            OperationError::MissingArgument { .. } => 500,
            OperationError::TelegramAPIError { .. } => 600,
            OperationError::SerdeJsonError(_) => 700,
        }
    }
}
