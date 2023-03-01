// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::process;

use clap::Parser;
use log::error;
use tgcli::cli::{match_app, Cli};

fn main() {
    let cli = Cli::parse();
    match match_app(cli) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
            process::exit(e.exit_code());
        }
    }
}
