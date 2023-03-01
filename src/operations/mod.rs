use std::{error, fmt};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod bot;

#[derive(Debug)]
pub(crate) struct RootParams {
    #[allow(dead_code)]
    verbosity: clap_verbosity_flag::Verbosity,
}

impl RootParams {
    pub(crate) fn new(verbosity: clap_verbosity_flag::Verbosity) -> Self {
        Self { verbosity }
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
