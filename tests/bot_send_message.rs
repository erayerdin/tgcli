use std::env;

use assert_cmd::Command;
use tgcli::operations::CommonExitCodes;

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
fn send_message_to_present_receiver(mut binary: Command) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "message",
            "foo",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
        ])
        .assert();

    assertion.success();
}

#[rstest]
fn send_message_to_absent_receiver(mut binary: Command) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "message",
            "foo",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER_ABSENT")
                .expect("TELEGRAM_RECEIVER_ABSENT environment variable could not be found. Please create .env file and define it."),
        ])
        .assert();

    assertion
        .failure()
        .code(CommonExitCodes::TelegramAPIBadRequest as i32);
}

#[rstest]
fn send_message_format(mut binary: Command, #[values("markdown", "html")] format: &str) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "message",
            if format == "html" {
                r#"<strong>foo</strong>"#
            } else {
                "*foo*"
            },
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
            "--format",
            format,
        ])
        .assert();

    assertion.success();
}
