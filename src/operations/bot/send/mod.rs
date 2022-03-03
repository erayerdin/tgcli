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

pub(crate) mod audio;
pub(crate) mod document;
pub(crate) mod location;
pub(crate) mod message;
pub(crate) mod photo;
pub(crate) mod poll;
pub(crate) mod video;

#[derive(Debug, Clone, ArgEnum)]
pub(crate) enum MessageFormat {
    Markdown,
    HTML,
}

#[derive(Debug)]
pub(crate) struct SendParams {
    pub(crate) receiver: String,
    pub(crate) format: MessageFormat,
    pub(crate) silent: bool,
}

impl SendParams {
    pub(crate) fn new(receiver: String, format: MessageFormat, silent: bool) -> Self {
        Self {
            receiver,
            format,
            silent,
        }
    }
}

pub(crate) trait SendOperation {
    fn send(self) -> Result<(), OperationError>;
}
