use crate::operations::{bot::BotParams, RootParams};

use super::{SendOperation, SendParams};

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
pub struct LocationParams {
    latitude: f32,
    longitude: f32,
}

impl LocationParams {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Debug)]
pub struct SendLocationOperation {
    params: (RootParams, BotParams, SendParams, LocationParams),
}

impl SendLocationOperation {
    pub fn new(params: (RootParams, BotParams, SendParams, LocationParams)) -> Self {
        Self { params }
    }
}

impl SendOperation for SendLocationOperation {
    fn send(self) -> Result<(), crate::operations::OperationError> {
        todo!() // TODO implement SendLocationOperation
    }
}
