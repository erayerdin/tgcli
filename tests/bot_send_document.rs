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
fn send_document(mut binary: Command) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "document",
            "resources/test/doc.txt",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it.")
        ])
        .assert();

    assertion.success();
}

#[rstest]
fn send_document_with_message(mut binary: Command) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "document",
            "resources/test/doc.txt",
            "--message",
            "example document",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it.")
        ])
        .assert();

    assertion.success();
}

#[rstest]
fn send_document_with_long_message(mut binary: Command) {
    let msg = (0..1034).map(|_| "a").collect::<String>();

    let assertion = binary
        .args([
            "bot",
            "send",
            "document",
            "resources/test/doc.txt",
            "--message",
            &msg,
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it.")
        ])
        .assert();

    assertion.failure().code(1); // Validation error code from clap on message
}
