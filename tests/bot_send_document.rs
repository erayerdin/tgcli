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
fn send_document(mut binary: Command) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
            "document",
            "resources/test/doc.txt",
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
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
            "document",
            "resources/test/doc.txt",
            "--message",
            "example document",
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
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
            "document",
            "resources/test/doc.txt",
            "--message",
            &msg,
        ])
        .assert();

    assertion.failure();
}

#[rstest]
fn send_document_with_thumbnail(mut binary: Command) {
    let assertion = binary
        .args([
            "bot",
            "send",
            "--receiver",
            &env::var("TELEGRAM_RECEIVER")
                .expect("TELEGRAM_RECEIVER environment variable could not be found. Please create .env file and define it."),
            "document",
            "resources/test/doc.txt",
            "--thumbnail",
            "resources/test/thumbnail512.png",
        ])
        .assert();

    assertion.success();
}
