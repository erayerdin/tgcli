use std::convert::TryFrom;

use reqwest::blocking::multipart::Form;

use crate::operations::{CommonExitCodes, OperationError};

use super::{ChatId, InputFile, ParseMode};

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

#[derive(Debug)]
pub struct SendVideoRequestModel {
    chat_id: ChatId,
    video: InputFile,
    width: Option<usize>,
    height: Option<usize>,
    caption: Option<String>,
    parse_mode: ParseMode,
}

impl TryFrom<SendVideoRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendVideoRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting to SendVideoRequestModel to Form...");
        let chat_id = m.chat_id.to_string();
        let parse_mode = m.parse_mode.to_string();

        let initial_form = Form::new()
            .text("chat_id", chat_id)
            .text("parse_mode", parse_mode);

        let caption_form = match m.caption {
            Some(c) => initial_form.text("caption", c),
            None => initial_form,
        };

        let video_form = match m.video {
            InputFile::Local(p) => match caption_form.file("video", p) {
                Ok(f) => f,
                Err(e) => {
                    return Err(OperationError::new(
                        CommonExitCodes::ReqwestFormError as i32,
                        &format!("Could not send video to Telegram. {}", e),
                    ))
                }
            },
            InputFile::Remote(u) => caption_form.text("video", u.to_string()),
            InputFile::Id(i) => caption_form.text("video", i),
        };

        let width_form = match m.width {
            Some(w) => video_form.text("width", w.to_string()),
            None => video_form,
        };

        let height_form = match m.height {
            Some(h) => video_form.text("height", h.to_string()),
            None => width_form,
        };

        Ok(height_form)
    }
}

        Ok(dimension_form)
    }
}
