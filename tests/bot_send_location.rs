use std::env;

use assert_cmd::Command;

// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate rstest;

#[fixture]
fn binary() -> Command {
    let _ = dotenv::dotenv();
    Command::cargo_bin("tgcli").expect("Could not find tgcli binary.")
}

#[rstest]
fn send_location(mut binary: Command) {
    let assertion = binary.args([
        "bot",
        "send",
        "--receiver",
        &env::var("TELEGRAM_RECEIVER")
            .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
        "location",
        "-x", "38.42",
        "-y", "27.14",
    ]).assert();

    assertion.success();
}

#[rstest]
fn send_location_invalid_float(mut binary: Command, #[values("a", "0..1", "1..0")] latitude: &str) {
    let assertion = binary.args([
        "bot",
        "send",
        "--receiver",
        &env::var("TELEGRAM_RECEIVER")
            .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
        "location",
        "-x", latitude,
        "-y", "27.14",
    ]).assert();

    assertion.failure();
}
