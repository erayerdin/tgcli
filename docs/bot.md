# Bot

## Introduction

Bots automate messaging in Telegram. They immitate an account / real person.
However, their username always finishes with `bot` suffix. In `tgcli`, bot
actions can be invoked under `tgcli bot` subcommand. To get offline help, use:

```bash
tgcli bot --help
```

`bot` subcommand also has flags that you might be interested.

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-t | --token | Required[^1] | Token of bot.

[^1]: It is not required if you have `TELEGRAM_BOT_TOKEN` environment variable
      set in your current shell session.

!!! tip
    You can also set `TELEGRAM_BOT_TOKEN` environment variable to current
    session of your terminal in order to protect your token from being exposed
    regularly.

        tgcli bot -t "YourBotToken" send --receiver "somebody" "message"
        # or better
        export TELEGRAM_BOT_TOKEN="YourBotToken"
        tgcli bot send --receiver "somebody" "message"

## send

`send` is a subcommand of `bot` which, hence the name, invoke sending
operations. To get help:

    tgcli bot send --help

`send` has the flags below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-r | --receiver | Required | The receiver's ID, an integer.
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.

After you define the receiver's ID, then you can use any subcommand of `send`.
To give an example:

```bash
# assuming receiver id is 1234
tgcli bot send -r 1234 message "Hello, world!"
# or --receiver
```

!!! note
    `-r`/`--receiver` is a *required* argument. You *have to* define it in
    order to use any subcommand of `send`, *even if what you need is only
    `--help`*.

!!! tip
    ID of a user  *is not* username or human-readable name. It is an unsigned
    64-bit integer representing the account. To get your ID, send
    [@userinfobot](https://t.me/userinfobot) *any* message and it will provide
    you *your own* user id.

### message

`message` is a subcommand of `send` command and is used to send *regular*
messages. To get help:

```bash
tgcli bot send --receiver 1234 message --help
```

`message` has the flags below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
 | message | Required | The message.

In order to send a message, do:

```bash
tgcli bot send --receiver 1234 message "foo"
```

`--format` helps you define the format of your message. See the example:

```bash
tgcli bot send --receiver 1234 message "<b>bold</b>" --format html
```

!!! warning
    Since Telegram also targets the mobile environment, it is safe to assume
    that not all features and/or tags of markdown and/or HTML are supported.
    Before using different features or tags, see
    [this part of the official bot API documentation][telegram_bot_api_markdown]
    for *Markdown* and
    [this part of the official bot API documentation][telegram_bot_api_html] for
    *HTML* in order to review the limitations.

[telegram_bot_api_markdown]: https://core.telegram.org/bots/api#markdown-style
[telegram_bot_api_html]: https://core.telegram.org/bots/api#html-style

### document

`document` is a subcommand of `send` and is used to send files through `tgcli`.
To get help:

```bash
tgcli bot send --receiver 1234 document --help
```

!!! Warning
    A file sent by `document` subcommand is *only downloadable*, which means it
    will not have traits of several media types in Telegram such as play button,
    full-screen view or keyboard navigation etc.

`document` owns the flags below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
 | file | Required | Path to file.
-m | --message | Optional | The message.
 | --thumbnail | Optional | An image file to set thumbnail.

In order to send a file, do:

```bash
tgcli bot send --receiver 1234 document path/to/file
```

#### File Storage Limits

The file storage limit for `document` is 50 megabytes
for other files
[as stated in the documentation](https://core.telegram.org/bots/api#sending-files)
Also, when you send a file, the message is limited to have up to *only 1024
characters*.

While we don't know how long the files are kept in the server, it is safe to
assume that Telegram server will wipe files depending on:

 - when they are accessed,
 - if they are forwarded or saved and/or
 - when the last login of forwarders or savers was etc.

And it is even *safer to assume that bots' files will have higher priority in
wiping operations*. That's why it is a good practice to forward the files sent
by bots to *Saved Messages*, even better to backup them to a storage that you
own if these files have higher importance to you.

### photo

`photo` is a subcommand of `send` and is used to send photos
through `tgcli`. To get help:

```bash
tgcli bot send --receiver 1234 photo --help
```

!!! info
    A file sent by `photo` subcommand (i) can be viewed
    **full-screen** *with one click/touch* and (ii)
    **can be navigated** *with arrow keys on the keyboard or
    swiping on the touch screen*.

`photo` subcommand owns these flags:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
 | file | Required | Path to file.
-m | --message | Optional | The message.

The usage is similar to the usage of [document](bot.md#document).

### video

`video` is a subcommand of `send` and is used to send videos
through `tgcli`. To get help:

```bash
tgcli bot send --receiver 1234 video --help
```

!!! info
    A file sent by `video` subcommand can be viewed
    **full-screen** *with a button*.

`video` has the flags below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
 | file | Required | Path to file.
-m | --message | Optional | The message.
-w | --width | Optional | The width of the video.
-v | --vertical | Optional | The height of the video.

The usage is similar to the usage of [document](bot.md#document).

!!! warning
    Defining width and height with their related flags do not actually change
    the resolution of the video. It only shapes the video container with the
    aspect ratio of the width and height in the application. For more
    information, see [#27][issue_27] and [#40][issue_40].

[issue_27]: https://github.com/erayerdin/tgcli/issues/27
[issue_40]: https://github.com/erayerdin/tgcli/issues/40

### audio

`audio` is a subcommand of `send` and is used to send audios
through `tgcli`. To get help:

```bash
tgcli bot send --receiver 1234 audio --help
```

!!! info
    A file sent by `audio` subcommand **can be played** *with a
    play button*.

`audio` owns the flags below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
 | file | Required | Path to file.
-m | --message | Optional | The message.
 | --performer | Optional | The performer of audio.
 | --title | Optional | The title of audio.

The usage is similar to the usage of [document](bot.md#document).

### poll

<!-- TODO update docs -->

`poll` is a subcommand of `send` and is used to send polls. To get help:

```bash
tgcli bot send --receiver 1234 poll --help
```

`poll` has these flags:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
 | question | Required | The question for poll.
-o | --option | Required[^2] | A single option for poll. You can define multiple options.

!!! warning
    You cannot send polls to private chats but only to groups and channels.

To start a poll:

```bash
tgcli bot send --receiver 1234 poll "Am I a ghost?" -o "Yes" -o "No"
```

!!! note
    Also keep in mind the the order of `-o`/`--option` is preserved for polls.

[^2]: You need to define at least two options for a valid poll.

### location

`location` is a subcommand of `send` and is used to send a location. To get
help:

```bash
tgcli bot send --receiver 1234 location --help
```

`location` owns the flags below:

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-x | --latitude | Required | Latitude on the world map. A float.
-y | --longitude | Required | Longtitude on the world map. A float.
