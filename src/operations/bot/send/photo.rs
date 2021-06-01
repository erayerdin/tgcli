use std::path::PathBuf;

use super::SendOperation;

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

pub struct PhotoParams {
    file: PathBuf,
    message: Option<String>,
}

impl PhotoParams {
    pub fn new(file: PathBuf, message: Option<String>) -> Self {
        Self { file, message }
    }
}

pub struct SendPhotoOperation {
    params: PhotoParams,
}

impl SendPhotoOperation {
    pub fn new(params: PhotoParams) -> Self {
        Self { params }
    }
}

impl SendOperation for SendPhotoOperation {
    fn send(self) -> Result<(), crate::operations::OperationError> {
        todo!() // TODO implement SendPhotoOperation
    }
}
