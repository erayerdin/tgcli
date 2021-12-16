use std::{path, string};

use reqwest::{multipart::Part, Body};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::operations::{CommonExitCodes, OperationError};

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

pub mod sendaudio;
pub mod senddocument;
pub mod sendlocation;
pub mod sendmessage;
pub mod sendphoto;
pub mod sendpoll;
pub mod sendvideo;

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

fn generate_form_part_from_file(location: path::PathBuf) -> Result<Part, OperationError> {
    // REF https://github.com/seanmonstar/reqwest/issues/646#issuecomment-616985015

    let file = match tokio::fs::File::open(location.clone()) {
        Ok(f) => f,
        Err(e) => {
            return Err(OperationError::new(
                CommonExitCodes::TokioFsFileError as i32,
                "An error occurred while trying to read the input file.",
                Some(e),
            ))
        }
    };

    let body = Body::wrap_stream(FramedRead::new(file, BytesCodec::new()));

    let filename = match location
        .file_name()
        .map(|val| val.to_string_lossy().to_string())
    {
        Some(n) => n,
        None => {
            return Err(OperationError::new(
                CommonExitCodes::StdFsInvalidFilename as i32,
                "Could not get filename from input file.",
                None::<&str>,
            ))
        }
    };

    Ok(Part::stream(body).file_name(filename))
}
