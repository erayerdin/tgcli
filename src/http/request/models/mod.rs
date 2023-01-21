use std::{path, string};

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod sendaudio;
pub(crate) mod senddocument;
pub(crate) mod sendlocation;
pub(crate) mod sendmessage;
pub(crate) mod sendphoto;
pub(crate) mod sendpoll;
pub(crate) mod sendvideo;

#[derive(Debug)]
/// What the type of ChatId is.
enum ChatId {
    Int(usize),
    Str(String),
}

impl string::ToString for ChatId {
    fn to_string(&self) -> String {
        match self {
            ChatId::Int(v) => v.to_string(),
            ChatId::Str(v) => v.clone(),
        }
    }
}

#[derive(Debug)]
/// Which format Telegram should handle the message text in.
enum ParseMode {
    Markdown,
    HTML,
}

impl string::ToString for ParseMode {
    fn to_string(&self) -> String {
        match self {
            ParseMode::Markdown => "MarkdownV2".to_owned(),
            ParseMode::HTML => "HTML".to_owned(),
        }
    }
}

// TODO remote and id will be implemented
#[allow(dead_code)]
#[derive(Debug)]
/// The file that will be sent to Telegram.
enum InputFile {
    /// Local file.
    Local(path::PathBuf),
    /// Remote file URL.
    Remote(url::Url),
    /// The id of file that was sent before.
    Id(String),
}
