// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate clap;

pub mod cli;
mod http;
pub(crate) mod operations;

const API_ROOT_URL: &str = "https://api.telegram.org/bot";
