# Bot

Bots automate messaging in Telegram. They immitate an account / real person.
However, their username always finishes with `bot` suffix. In `tgcli`, bot
actions are executed under `tgcli bot` command. To get offline help, use:

    tgcli bot --help

`bot` subcommand also has options that you might be interested.

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-t | --token | Required<sup>1</sup> | Token of bot.

<small>**1:** You can also pass `TELEGRAM_BOT_TOKEN` environment variable to
current session of your terminal in order to protect your token from being
exposed regularly.</small>

    tgcli bot -t "YourBotToken" send -r "somebody" "message"
    # or better
    export TELEGRAM_BOT_TOKEN="YourBotToken"
    tgcli bot send -r "somebody" "message"

## send

A subcommand of `bot` to send regular messages to any person, to get help:

    tgcli bot send --help

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-t | --token | Required<sup>1</sup> | Token of bot.
-r | --receiver | Required | The receiver's ID. An integer.
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
-f | --file<sup>1</sup> | Optional | File to send.
 | message | Required | The message.

<small>**1:** [Telegram bot API documentation](https://core.telegram.org/bots/api#senddocument)
clearly states that bots can send files up to 50 megabytes and character limit
of the message is limited to 1024 characters when a file is sent.</small>

 > #### Tip
 > ID of a user  *is not* username or human-readable name. It is an integer
 > representing the account. To get your ID, send
 > [@userinfobot](https://t.me/userinfobot) *any* message.

### Examples

    # assuming you have TELEGRAM_BOT_TOKEN environment variable

    # a regular message
    tgcli bot send -r "123456" "My *markdown* message."

    # an HTML-flavored message
    tgcli bot send -r "123456" --format "html" "My <strong>HTML</strong> message."

    # send file
    export TELEGRAM_BOT_TOKEN="BotToken"
    tgcli bot send -r "123456" -f "any.txt" "File description"
