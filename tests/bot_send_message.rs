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
fn send_message_to_present_receiver(mut binary: Command) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
            "message",
            "foo",
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
            "--receiver",
            &env::var("TELEGRAM_RECEIVER_ABSENT")
                .expect("TELEGRAM_RECEIVER_ABSENT environment variable could not be found. Please create .env file and define it."),
            "message",
            "foo",
        ])
        .assert();

    assertion.failure();
}

#[rstest]
fn send_message_format(mut binary: Command, #[values("markdown", "html")] format: &str) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
            "--format",
            format,
            "message",
            if format == "html" {
                r#"<strong>foo</strong>"#
            } else {
                "*foo*"
            },
        ])
        .assert();

    assertion.success();
}

#[rstest]
fn send_message_silent(mut binary: Command) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
            "--silent",
            "message",
            "foo",
        ])
        .assert();

    assertion.success();
}

#[rstest]
fn send_message_protect_content(mut binary: Command) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
            "--protect-content",
            "message",
            "protected message",
        ])
        .assert();

    assertion.success();
}
