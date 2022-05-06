use crate::operations::OperationError;

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
    pub(crate) protect_content: bool,
}

impl SendParams {
    pub(crate) fn new(
        receiver: String,
        format: MessageFormat,
        silent: bool,
        protect_content: bool,
    ) -> Self {
        Self {
            receiver,
            format,
            silent,
            protect_content,
        }
    }
}

pub(crate) trait SendOperation {
    fn send(self) -> Result<(), OperationError>;
}
