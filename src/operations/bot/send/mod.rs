use crate::operations::OperationError;

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

pub mod audio;
pub mod document;
pub mod message;
pub mod photo;
pub mod video;

pub enum MessageFormat {
    Markdown,
    HTML,
}

pub struct SendParams {
    receiver: String,
    format: MessageFormat,
}

impl SendParams {
    pub fn new(receiver: &str, format: MessageFormat) -> Self {
        Self {
            receiver: String::from(receiver),
            format,
        }
    }
}

pub trait SendOperation {
    fn send(self) -> Result<(), OperationError>;
}
