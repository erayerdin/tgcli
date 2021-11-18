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

pub mod models;

#[macro_export]
macro_rules! handle_response {
    ($response:ident, on_success => $success:expr, on_failure => $failure:expr) => {
        match $response {
            Ok(r) => {
                if r.status().is_success() {
                    trace!("response: {:?}", r);
                    $success;
                    Ok(())
                } else {
                    use crate::http::response::models::{
                        message::MessageModel, GenericResponseModel,
                    };
                    $failure;
                    match r.json::<GenericResponseModel<MessageModel>>() {
                        Ok(i) => match i.description {
                            Some(d) => Err(OperationError::new(
                                CommonExitCodes::TelegramAPIBadRequest as i32,
                                &d,
                            )),
                            None => Err(OperationError::new(
                                CommonExitCodes::TelegramAPIMissingDescription as i32,
                                "No description was provided by Telegram for this error.",
                            )),
                        },
                        Err(e) => Err(OperationError::new(
                            CommonExitCodes::SerdeDeserializationError as i32,
                            &format!("An error occurred while deserializing the response. {}", e),
                        )),
                    }
                }
            }
            Err(e) => {
                $failure;
                Err(OperationError::new(
                    CommonExitCodes::ReqwestConnectionError as i32,
                    &format!("An error occured while connecting to Telegram API. {}", e),
                ))
            }
        }
    };
}
