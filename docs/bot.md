# Bot

## Introduction

Bots automate messaging in Telegram. They immitate an account / real person. However, their username always finishes with `bot` suffix. In `tgcli`, bot actions can be invoked under `tgcli bot` subcommand. To get offline help, use:

```bash
tgcli bot --help
```

`bot` subcommand also has arguments that you might be interested.

| Short Flag | Full Flag | Required/Optional |  Description   |
| ---------- | --------- | ----------------- |  ------------- |
| -t         | --token   | Required[^1]      |  Token of bot. |

[^1]: It is not required if you have `TELEGRAM_BOT_TOKEN` environment variable
      set in your current shell session.

!!! tip
    You can also set `TELEGRAM_BOT_TOKEN` environment variable to current session of your terminal in order to protect your token from being exposed regularly.

        tgcli bot send message "hello" -t "YourBotToken" --receiver "somebody"
        # or better
        export TELEGRAM_BOT_TOKEN="YourBotToken"
        tgcli bot send message "hello" --receiver "somebody"
        # or even better
        TELEGRAM_BOT_TOKEN="YourBotToken" tgcli bot send message "hello" --receiver "somebody"

## send

`send` is a subcommand of `bot` which, hence the name, invoke sending
operations. To get help:

    tgcli bot send --help

`send` has the arguments below:

| Short Flag | Full Flag  | Required/Optional | Description                                                                                               |
|------------|------------|-------------------|-----------------------------------------------------------------------------------------------------------|
| -r         | --receiver | Required          | The receiver's ID, an integer.                                                                            |
|            | --format   | Optional          | The format of message. Choices are `markdown` and `html`. Default is `markdown`.[^markdown_format_choice] |
|            | --silent   | Optional          | The message will not play notification sound on target device if present.                                 |

After you define the receiver's ID, then you can use any subcommand of `send`. To give an example:

```bash
# assuming receiver id is 1234
tgcli bot send -r 1234 message "Hello, world!"
# or --receiver
```

!!! tip
    ID of a user  *is not* username or human-readable name. It is an unsigned 64-bit integer representing the account. To get your ID, send [@userinfobot](https://t.me/userinfobot) *any* message and it will provide you *your own* user id.

If you'd like to send a message without the notification sound playing on the target device, you can use `--silent` global argument to supress the sound.

```bash
tgcli bot send message "foo" -r 1234 --silent
```

!!! warning
    `--silent` argument does not disable notification, it only supresses the notification sound. The user will still see the notification on device *unless the user willingly disabled the notifications from your bot*.

[^markdown_format_choice]: By default, [MarkdownV2](https://core.telegram.org/bots/api#markdownv2-style) style is used.

### message

`message` is a subcommand of `send` command and is used to send *regular* messages. To get help:

```bash
tgcli bot send message --help
```

`message` has the arguments below:

| Short Flag | Full Flag | Required/Optional | Description                 |
| ---------- | --------- | ----------------- | --------------------------- |
| message    | Required  | Required          | The content of the message. |

In order to send a message, do:

```bash
tgcli bot send --receiver 1234 message "foo"
```

`--format` helps you define the format of your message. See the example:

```bash
tgcli bot send --receiver 1234 --format html message "<b>bold</b>"
```

!!! warning
    Since Telegram also targets the mobile environment, it is safe to assume that not all features and/or tags of markdown and/or HTML are supported. Before using different features or tags, see [this part of the official bot API documentation][telegram_bot_api_markdown] for *Markdown* and [this part of the official bot API documentation][telegram_bot_api_html] for *HTML* in order to review the limitations.

[telegram_bot_api_markdown]: https://core.telegram.org/bots/api#markdownv2-style
[telegram_bot_api_html]: https://core.telegram.org/bots/api#html-style

### document

`document` is a subcommand of `send` and is used to send files through `tgcli`. To get help:

```bash
tgcli bot send document --help
```

!!! Tip
    A file sent by `document` subcommand is *only downloadable*, which means it will not have traits of several media types in Telegram such as play button, full-screen view or keyboard navigation etc.

`document` owns the arguments below:

| Short Flag | Full Flag   | Required/Optional | Description                       |
| ---------- | ----------- | ----------------- | --------------------------------- |
|            | file        | Required          | Path to file.                     |
| -m         | --message   | Optional          | The message[^doc_msg_char_limit]. |
|            | --thumbnail | Optional          | An image file to set thumbnail.   |

In order to send a file, do:

```bash
tgcli bot send --receiver 1234 document path/to/file
```

[^doc_msg_char_limit]: The current limit for messages on documents, videos, audios or photos is limited to at most 1024 characters by Telegram. You can see caption fields of all document-related endpoints, [such as this one](https://core.telegram.org/bots/api#sendphoto).

#### File Storage Limits

The file storage limit for `document` is 50 megabytes for other files [as stated in the documentation](https://core.telegram.org/bots/api#sending-files).

While we don't know how long the files are kept in the server, it is safe to assume that Telegram server will wipe files depending on:

 - when the last time they were accessed was,
 - their forward/save count
 - the last login time of forwarders or savers was etc.

And it is even *safer to assume that bots' files will have higher priority in wiping operations*[^no_report_on_file_wiping]. That's why it is a good practice to forward the files sent by bots to *Saved Messages*, even better to backup them to a storage that you own if these files have higher importance to you.

[^no_report_on_file_wiping]: One should keep in mind that there have been few reports of data loss complaint from Telegram's side to this day.

### photo

`photo` is a subcommand of `send` and is used to send photos through `tgcli`. To get help:

```bash
tgcli bot send photo --help
```

!!! info
    A file sent by `photo` subcommand has these traits:
    
    1. It can be viewed **full-screen** *with one click/touch*.
    2. It can be **navigated** *with arrow keys on the keyboard or
    swiping on the touch screen*.

`photo` subcommand owns these arguments:

| Short Flag | Full Flag | Required/Optional | Description   |
| ---------- | --------- | ----------------- | ------------- |
|            | file      | Required          | Path to file. |
| -m         | --message | Optional          | The message.  |

The usage is similar to the usage of [document](bot.md#document).

### video

`video` is a subcommand of `send` and is used to send videos through `tgcli`. To get help:

```bash
tgcli bot send video --help
```

!!! info
    A file sent by `video` subcommand can be viewed **full-screen** *with a button*.

`video` has the arguments below:

| Short Flag | Full Flag | Required/Optional | Description   |
| ---------- | --------- | ----------------- | ------------- |
|            | file      | Required          | Path to file. |
| -m         | --message | Optional          | The message.  |

The usage is similar to the usage of [document](bot.md#document).

### audio

`audio` is a subcommand of `send` and is used to send audios through `tgcli`. To get help:

```bash
tgcli bot send audio --help
```

!!! info
    A file sent by `audio` subcommand **can be played** *with a play button*.

`audio` owns the arguments below:

| Short Flag | Full Flag   | Required/Optional | Description             |
| ---------- | ----------- | ----------------- | ----------------------- |
|            | file        | Required          | Path to file.           |
| -m         | --message   | Optional          | The message.            |
|            | --performer | Optional          | The performer of audio. |
|            | --title     | Optional          | The title of audio.     |

The usage is similar to the usage of [document](bot.md#document).

### poll

`poll` is a subcommand of `send` and is used to send polls. To get help:

```bash
tgcli bot send poll --help
```

`poll` has these arguments:

| Short Flag | Full Flag | Required/Optional | Description                                                |
| ---------- | --------- | ----------------- | ---------------------------------------------------------- |
|            | question  | Required          | The question for poll.                                     |
| -o         | --option  | Required[^2]      | A single option for poll. You can define multiple options. |

To start a poll:

```bash
# a plain poll
tgcli bot send --receiver 1234 poll "Am I a ghost?" -o "Yes" -o "No"
```

!!! note
    Also keep in mind the the order of `-o`/`--option` is preserved for polls.

[^2]: You need to define at least two options for a valid poll.

### location

`location` is a subcommand of `send` and is used to send a location. To get help:

```bash
tgcli bot send location --help
```

`location` owns the arguments below:

| Short Flag | Full Flag   | Required/Optional | Description                           |
| ---------- | ----------- | ----------------- | ------------------------------------- |
| -x         | --latitude  | Required          | Latitude on the world map. A float.   |
| -y         | --longitude | Required          | Longtitude on the world map. A float. |
