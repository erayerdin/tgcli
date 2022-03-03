use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;

use crate::operations::{
    bot::send::{self, document::SendDocumentParams},
    OperationError,
};

use super::{ChatId, InputFile, ParseMode};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug)]
/// A model for /sendDocument request.
pub(crate) struct SendDocumentRequestModel {
    chat_id: ChatId,
    document: InputFile,
    thumbnail: Option<InputFile>,
    caption: Option<String>,
    parse_mode: ParseMode,
    disable_notification: bool,
}

impl TryFrom<SendDocumentRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendDocumentRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting SendDocumentRequestModel to Form...");
        let chat_id = m.chat_id.to_string();
        let parse_mode = m.parse_mode.to_string();

        let initial_form = Form::new()
            .text("chat_id", chat_id)
            .text("parse_mode", parse_mode);

        let caption_form = match m.caption {
            Some(c) => initial_form.text("caption", c),
            None => initial_form,
        };

        let document_form = match m.document {
            InputFile::Local(p) => caption_form
                .file("document", p)
                .map_err(|err| OperationError::IoError(err))?,
            InputFile::Remote(u) => caption_form.text("document", u.to_string()),
            InputFile::Id(i) => caption_form.text("document", i),
        };

        let thumbnail_form = match m.thumbnail {
            Some(inputfile) => match inputfile {
                InputFile::Local(p) => document_form
                    .file("thumbnail", p)
                    .map_err(|err| OperationError::IoError(err))?,
                InputFile::Remote(u) => document_form.text("thumbnail", u.to_string()),
                InputFile::Id(i) => document_form.text("thumbnail", i),
            },
            None => document_form,
        };

        let notification_form = match m.disable_notification {
            true => thumbnail_form.text("disable_notification", "true"),
            false => thumbnail_form,
        };

        Ok(notification_form)
    }
}

impl From<SendDocumentParams> for SendDocumentRequestModel {
    fn from(params: SendDocumentParams) -> Self {
        debug!("Converting SendDocumentParams to SendDocumentRequestModel...");

        let chat_id = match params.2.receiver.parse::<usize>() {
            Ok(v) => ChatId::Int(v),
            Err(_) => ChatId::Str(params.2.receiver),
        };

        let caption = params.3.message;

        let parse_mode = match params.2.format {
            send::MessageFormat::Markdown => ParseMode::Markdown,
            send::MessageFormat::HTML => ParseMode::HTML,
        };

        let document = InputFile::Local(params.3.file);

        let thumbnail = match params.3.thumbnail {
            Some(p) => Some(InputFile::Local(p)),
            None => None,
        };

        let disable_notification = params.2.silent;

        SendDocumentRequestModel {
            chat_id,
            caption,
            parse_mode,
            document,
            thumbnail,
            disable_notification,
        }
    }
}
