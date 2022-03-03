use std::env;

use assert_cmd::Command;

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
