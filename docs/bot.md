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

[^1]: It is not required if you have `TELEGRAM_BOT_TOKEN` set in your current
      shell session.

!!! tip
    You can also pass `TELEGRAM_BOT_TOKEN` environment variable to current
    session of your terminal in order to protect your token from being exposed
    regularly.

        tgcli bot -t "YourBotToken" send -r "somebody" "message"
        # or better
        export TELEGRAM_BOT_TOKEN="YourBotToken"
        tgcli bot send -r "somebody" "message"

## send

A subcommand of `bot` to send regular messages to any person, to get help:

    tgcli bot send --help

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-t | --token | Required[^2] | Token of bot.
-r | --receiver | Required | The receiver's ID. An integer.
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
-f | --file[^2] | Optional | File to send.
 | --as | Optional | Type of the file that is sent. Choices are `document`, `video`, `audio` and `photo`. Default is `document`.
 | message | Required | The message.

[^2]: File storage in Telegram server is a bit complex topic, which is
      discussed in [file storage and media types section](bot.md#file-storage-and-media-types)
      of the documentation.

!!! tip
    ID of a user  *is not* username or human-readable name. It is an integer
    representing the account. To get your ID, send
    [@userinfobot](https://t.me/userinfobot) *any* message.

## File Storage and Media Types

### Notes on Limits of File Storage

The file storage limit for a bot up to 10 megabytes for photos and 50 megabytes
for other files
[as stated in the documentation](https://core.telegram.org/bots/api#sending-files)
Also, when you send a file, the message is limited to have up to 1024
characters for all media types.

While we don't know how long the files are kept in the server, it is safe to
assume that Telegram server will wipe files depending on when they are accessed,
if they are forwarded or saved and when the last login of forwarders or savers
was. And it is even *safer to assume that bots' files will be higher priority in
wiping operations*. That's why it is a good practice to forward the files sent
by bots to *Saved Messages*, even better to backup them to a storage that you
own if these files have higher importance to you.

### Notes on Media Types

`document` is the default option for `--as` flag and it can be used for *all
types of files*. However, when you set `--as` to a different type, Telegram
constructs a custom message based on this type. While `document` type only has
a *download* button, `video` and `audio` have a *play* button and `video` and
`photo` can be displayed *full-screen*. That is why you might want to set `--as`
flag if you want further features.

Also, you should keep in mind that Telegram might *compress* the files, the
sending types of which are all except `document`. So, if you don't want the
loss of quality in your files, send them as `document` (which will lack
additional features like play button or showing full-screen).

Lastly, neither Telegram nor `tgcli` does not check mimetypes of the files you
send for performance reasons. So, you might come across weird quirks if you
send a file with a different types, such as an image having a play button, an
audio that can be displayed full-screen etc.

## Examples

    # assuming you have TELEGRAM_BOT_TOKEN environment variable

    # a regular message
    tgcli bot send -r "123456" "My *markdown* message."

    # an HTML-flavored message
    tgcli bot send -r "123456" --format "html" "My <strong>HTML</strong> message."

    # send file
    export TELEGRAM_BOT_TOKEN="BotToken"
    tgcli bot send -r "123456" -f "any.txt" "File description"

    # send file as
    tgcli bot send -r "123456" -f "any.png" --as "photo" "File description"
