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

pub mod bot;

#[derive(Debug)]
pub struct RootParams {
    secure: bool,
}

impl RootParams {
    pub fn new(secure: bool) -> Self {
        Self { secure }
    }
}

#[derive(Debug)]
pub struct OperationError {
    pub exit_code: i32,
    pub message: String,
}

impl OperationError {
    pub fn new(exit_code: i32, message: &str) -> Self {
        Self {
            exit_code,
            message: String::from(message),
        }
    }

    pub fn exit(self) {
        error!("{}", self.message);
        std::process::exit(self.exit_code);
    }
}

/// These are common exit codes that are used to exit
/// the application. -1 and 1 are reserved for Clap
/// itself.
///
/// All variant names start with where the error originates
/// from. If the error comes from clap, the names start with
/// Clap.
pub enum CommonExitCodes {
    // ////////// //
    // Std Errors //
    // ////////// //
    // between 2-19
    /// Provided value is not valid. For example,
    /// expected value is f32 while a non-f32 value
    /// is provided by the user.
    StdInvalidValue = 2,

    // /////////// //
    // Clap Errors //
    // /////////// //
    // between 20-39
    /// A required argument is not provided.
    ClapMissingValue = 20,

    // ////////////// //
    // Reqwest Errors //
    // ////////////// //
    // between 40-59
    /// An connection error occured.
    ReqwestConnectionError = 40,
    /// An error occured reported by the response.
    ReqwestHttpError = 41,
    /// An error occured related to constructing a form.
    ReqwestFormError = 42,

    // /////////////////// //
    // Telegram API Errors //
    // /////////////////// //
    // between 60-79
    TelegramAPIMissingDescription = 60,
    TelegramAPIBadRequest = 61,

    // //////////// //
    // Serde Errors //
    // //////////// //
    // between 80-99
    SerdeDeserializationError = 80,
}
