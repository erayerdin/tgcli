use std::path;

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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

pub(crate) fn image_validator(value: &str) -> ValidatorResult<path::PathBuf> {
    let path = file_presence_validator(value)?;
    let mimetype = mime_guess::from_path(&path)
        .first()
        .ok_or_else(|| "File type could not be recognized.".to_owned())?;

    if mimetype.type_() == mime::IMAGE {
        Ok(path)
    } else {
        Err("Provided file is not a valid image file.".to_owned())
    }
}

pub(crate) fn video_validator(value: &str) -> ValidatorResult<path::PathBuf> {
    let path = file_presence_validator(value)?;
    let mimetype = mime_guess::from_path(&path)
        .first()
        .ok_or_else(|| "File type could not be recognized.".to_owned())?;

    if mimetype.type_() == mime::VIDEO {
        Ok(path)
    } else {
        Err("Provided file is not a valid video file.".to_owned())
    }
}

pub(crate) fn audio_validator(value: &str) -> ValidatorResult<path::PathBuf> {
    let path = file_presence_validator(value)?;
    let mimetype = mime_guess::from_path(&path)
        .first()
        .ok_or_else(|| "File type could not be recognized.")?;

    if mimetype.type_() == mime::AUDIO {
        Ok(path)
    } else {
        Err("Provided file is not a valid audio file.".to_owned())
    }
}
