use std::path;

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

type ValidatorResult<T> = Result<T, String>;

pub(crate) fn caption_validator(value: &str) -> ValidatorResult<String> {
    if value.chars().count() > 1024 {
        return Err(String::from(
            "Message cannot be larger than 1024 characters.",
        ));
    }

    Ok(value.to_owned())
}

pub(crate) fn poll_question_validator(value: &str) -> ValidatorResult<String> {
    match value.len() {
        l if l < 1 || l > 300 => Err("The question length must be between 1 and 300.".to_owned()),
        _ => Ok(value.to_owned()),
    }
}

pub(crate) fn poll_option_validator(value: &str) -> ValidatorResult<String> {
    match value.len() {
        l if l < 1 || l > 100 => {
            return Err("The option length must be between 1 and 100.".to_owned())
        }
        _ => Ok(value.to_owned()),
    }
}

pub(crate) fn file_presence_validator(value: &str) -> ValidatorResult<path::PathBuf> {
    let path = path::PathBuf::from(value);

    if path.is_file() {
        Ok(path)
    } else {
        Err(format!("`{}` does not exist or is not a file.", value))
    }
}
