# Bot

## Introduction

Bots automate messaging in Telegram. They immitate an account / real person.
However, their username always finishes with `bot` suffix. In `tgcli`, bot
actions are executed under `tgcli bot` command. To get offline help, use:

    tgcli bot --help

`bot` subcommand also has options that you might be interested.

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-t | --token | Required[^1] | Token of bot.

[^1]: It is not required if you have `TELEGRAM_BOT_TOKEN` environment variable
      set in your current shell session.

!!! tip
    You can also pass `TELEGRAM_BOT_TOKEN` environment variable to current
    session of your terminal in order to protect your token from being exposed
    regularly.

        tgcli bot -t "YourBotToken" send -r "somebody" "message"
        # or better
        export TELEGRAM_BOT_TOKEN="YourBotToken"
        tgcli bot send -r "somebody" "message"

## send

`send` is a subcommand of `bot` which, hence the name, operate sending
operations. To get help:

    tgcli bot send --help

`send` has only one option:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-r | --receiver | Required | The receiver's ID, an integer.

After you define the receiver's ID, then you can use any subcommand of `send`.
To give an example:

    tgcli bot send -r $RECEIVER_ID message "Hello, world!"

Assuming you have replaced `$RECEIVER_ID` with a user or group id, this is how
you send a message.

!!! note
    You *have to* define `-r`/`--receiver`  in order to use any subcommand of
    `send`, even if what you need is only `--help`.

!!! tip
    ID of a user  *is not* username or human-readable name. It is an integer
    representing the account. To get your ID, send
    [@userinfobot](https://t.me/userinfobot) *any* message.

### message

`message` is a subcommand of `send` command and is used to send *regular*
messages. To get help:

    tgcli bot send -r $RECEIVER_ID message --help

`message` has the options and parameters below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
 | message | Required | The message.

In order to send a message, do:

    tgcli bot send -r $RECEIVER_ID message "foo"

`--format` helps you define the format of your message. See the example:

    tgcli bot send -r $RECEIVER_ID message "<b>bold</b>" --format html

!!! warning
    Since Telegram also targets the mobile environment, it is safe to assume
    that not all features and/or tags of markdown and/or HTML are supported.
    Before using different features or tags, see
    [this part of the official bot API documentation][telegram_bot_api_markdown]
    for *Markdown* and
    [this part of the official bot API documentation][telegram_bot_api_html] for
    *HTML* in order to see the limitations.

[telegram_bot_api_markdown]: https://core.telegram.org/bots/api#markdown-style
[telegram_bot_api_html]: https://core.telegram.org/bots/api#html-style

### file

`file` is a subcommand of `send` and is used to send files through `tgcli`. To
get help:

tgcli bot send -r $RECEIVER_ID file --help

`file` has the options and parameters below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-m | --message | Optional | The message.
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
 | --as | Optional | Type of the file that is sent. Choices are `document`, `video`, `audio` and `photo`. Default is `document`.
 | file | Required | Path to file.

In order to send a file, do:

    tgcli bot send -r $RECEIVER_ID file path/to/file

#### File Types

`document` is the default option for `--as` flag and it can be used for *all
types of files*. However, when you set `--as` to a different type, Telegram
constructs a custom message based on this type. While `document` type only has
a *download* button, `video` and `audio` have a *play* button and `video` and
`photo` can be displayed *full-screen*. That is why you might want to set `--as`
flag if you want further features on the client side. See the example:

    tgcli bot send -r $RECEIVER_ID file path/to/file.png --as photo

Also, you should keep in mind that Telegram might *compress* the files, the
types of which are all except `document`. So, if you don't want the loss of
quality in your files, send them as `document` (which will lack additional
features like play button or showing full-screen).

Lastly, neither Telegram nor `tgcli` checks mimetypes of the files you send for
performance reasons. So, you might come across weird quirks if you send a file
with a different types, such as an image having a play button, an audio that can
be displayed full-screen etc.

#### File Storage Limits

The file storage limit for a bot up to 10 megabytes for photos and 50 megabytes
for other files
[as stated in the documentation](https://core.telegram.org/bots/api#sending-files)
Also, when you send a file, the message is limited to have up to 1024
characters for all media types.

While we don't know how long the files are kept in the server, it is safe to
assume that Telegram server will wipe files depending on:
 - when they are accessed,
 - if they are forwarded or saved and/or
 - when the last login of forwarders or savers was
 - et cetera

And it is even *safer to assume that bots' files will be higher priority in
wiping operations*. That's why it is a good practice to forward the files sent
by bots to *Saved Messages*, even better to backup them to a storage that you
own if these files have higher importance to you.
