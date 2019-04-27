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

### document

`document` is a subcommand of `send` and is used to send files through `tgcli`. To
get help:

    tgcli bot send -r $RECEIVER_ID file --help

!!! Warning
    A file sent by `document` subcommand is *only
    downloadable*, which means it will not have traits of several
    media types in Telegram such as play button, full-screen view
    or keyboard navigation etc.

`document` has the options and parameters below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-m | --message | Optional | The message.
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
 | --thumbnail | Optional | An image file to set thumbnail.
 | file | Required | Path to file.

In order to send a file, do:

    tgcli bot send -r $RECEIVER_ID file path/to/file

#### File Storage Limits

The file storage limit for `document` is 50 megabytes
for other files
[as stated in the documentation](https://core.telegram.org/bots/api#sending-files)
Also, when you send a file, the message is limited to have up to 1024
characters.

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

### photo

`photo` is a subcommand of `send` and is used to send photos
through `tgcli`. To get help:

    tgcli bot send -r $RECEIVER_ID photo --help

!!! info
    A file sent by `photo` subcommand (i) can be viewed
    **full-screen** *with one click/touch* and (ii) is
    **can be navigated** *with arrow keys on the keyboard or
    swiping*.

`photo` has the options and parameters below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-m | --message | Optional | The message.
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
 | file | Required | Path to file.

The usage is similar to the usage of [document](bot.md#document).

### video

`video` is a subcommand of `send` and is used to send videos
through `tgcli`. To get help:

    tgcli bot send -r $RECEIVER_ID video --help

!!! info
    A file sent by `video` subcommand can be viewed
    **full-screen** *with a button*.

`video` has the options and parameters below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-m | --message | Optional | The message.
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
-h | --horizontal | Optional | The horizontal aspect ratio of video.
-v | --vertical | Optional | The vertical aspect ratio of video.
 | file | Required | Path to file.

The usage is similar to the usage of [document](bot.md#document).

!!! warning
    Telegram server assumes the aspect ratio of video as 1:1
    for thumbnail due to performance reasons. See
    [this issue][issue_27] for an example. That's why it is good
    to know the aspect ratio of the video beforehand. Standards
    are `16:9` for new videos and `4:3` for old videos.

[issue_27]: https://github.com/erayerdin/tgcli/issues/27

### audio

`audio` is a subcommand of `send` and is used to send audios
through `tgcli`. To get help:

    tgcli bot send -r $RECEIVER_ID audio --help

!!! info
    A file sent by `audio` subcommand **can be played** *with a
    play button*.

`audio` has the options and parameters below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-m | --message | Optional | The message.
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
 | --performer | Optional | The performer of audio.
 | --title | Optional | The title of audio.
 | file | Required | Path to file.

The usage is similar to the usage of [document](bot.md#document).

### poll

`poll` is a subcommand of `send` and is used to publish polls. To get help:

    tgcli bot send -r $RECEIVER_ID poll --help

`poll` has the options and parameters below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-o | --option | Required[^2] | A single option for poll. You can define multiple options.
 | question | Required | The question for poll.

!!! warning
    You cannot send polls to private chats but only to groups and channels.

To publish a poll:

    tgcli bot send -r $RECEIVER_ID poll "Am I a ghost?" -o "Yes" -o "No"

!!! note
    Also keep in mind the the order of `-o`/`--option` is preserved for polls.

[^2]: You need to define at least two options for a valid poll.

### location

`location` is a subcommand of `send` and is used to send a location. To get
help:

    tgcli bot send -r $RECEIVER_ID location --help

`location` has the options and parameters below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-x | --latitude | Required | Latitude on the world map. A float.
-y | --longitude | Required | Longtitude on the world map. A float.
