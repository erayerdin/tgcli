use std::{env::current_dir, path::PathBuf};

use mime::Name;
use mime_guess::MimeGuess;

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

type ValidatorResult = Result<(), String>;

pub fn caption_validator(value: String) -> ValidatorResult {
    if value.chars().count() > 1024 {
        return Err(String::from(
            "Message cannot be larger than 1024 characters.",
        ));
    }

    Ok(())
}

pub fn file_validator(value: String) -> ValidatorResult {
    let cwd = match current_dir() {
        Ok(d) => d,
        Err(_) => return Err(String::from("Could not get current working directory.")),
    };

    let path = if cwd.is_absolute() {
        if value.starts_with("~") {
            let home_expanded_path = shellexpand::tilde(&value).to_string();
            PathBuf::new().join(home_expanded_path)
        } else {
            cwd.join(value)
        }
    } else {
        PathBuf::from(value)
    };

    if path.exists() {
        if path.is_file() {
            Ok(())
        } else {
            Err(format!(
                "Provided path is not a file.\nPath: {}",
                path.to_string_lossy()
            ))
        }
    } else {
        Err(format!(
            "Provided path does not exist.\nPath: {}",
            path.to_string_lossy()
        ))
    }
}

fn validate_file_type(path: &PathBuf, file_type: Name) -> ValidatorResult {
    match MimeGuess::from_path(path).first() {
        Some(m) => {
            if m.type_() == file_type {
                Ok(())
            } else {
                Err(format!(
                    "The file is not a valid {file_type} file. Please provide a valid image file.\nPath: {}",
                    path.to_string_lossy(),
                    file_type = file_type,
                ))
            }
        }
        None => Err(format!(
            "Could not infer the file type. Please provide a valid {file_type} file.\nPath: {}",
            path.to_string_lossy(),
            file_type = file_type,
        )),
    }
}

pub fn image_validator(value: String) -> ValidatorResult {
    if let Err(val) = file_validator(value.clone()) {
        return Err(val);
    }

    let path = current_dir().unwrap().join(value);
    validate_file_type(&path, mime::IMAGE)
}

pub fn video_validator(value: String) -> ValidatorResult {
    if let Err(val) = file_validator(value.clone()) {
        return Err(val);
    }

    let path = current_dir().unwrap().join(value);
    validate_file_type(&path, mime::VIDEO)
}

pub fn audio_validator(value: String) -> ValidatorResult {
    if let Err(val) = file_validator(value.clone()) {
        return Err(val);
    }

    let path = current_dir().unwrap().join(value);
    validate_file_type(&path, mime::AUDIO)
}

pub fn positive_integer_validator(value: String) -> ValidatorResult {
    match value.parse::<usize>() {
        Ok(v) => {
            if v == 0 {
                Err(format!(
                    "Value must be a positive integer.\nValue: {}",
                    value
                ))
            } else {
                Ok(())
            }
        }
        Err(_) => Err(format!(
            "Value must be a positive integer.\nValue: {}",
            value
        )),
    }
}
