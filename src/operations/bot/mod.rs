// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod send;

#[derive(Debug)]
pub(crate) struct BotParams {
    token: String,
}

impl BotParams {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }
}
