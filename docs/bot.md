# Bot

Bots automate messaging in Telegram. In `tgcli`, bot actions are executed under
`tgcli bot` command. To get offline help, use:

    tgcli bot --help

## send

A subcommand of `bot` to send regular messages to any person, to get help:

    tgcli bot send --help

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-t | --token | Required<sup>1</sup> | Token of bot.
-r | --receiver | Required | The receiver's ID. An integer.
-f | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
 | message | Required | The message.

<small>1: You can also pass `TELEGRAM_BOT_TOKEN` environment variable to
current session of your terminal in order to protect your token from being
exposed regularly.</small>

 > #### Tip
 > ID of a user  *is not* username or human-readable name. It is an integer
 > representing the account. To get your ID, for example, send
 > [@userinfobot](https://t.me/userinfobot) *any* message.

### Examples

    # a regular message
    tgcli bot send -t "BotToken" -r "123456" "My *markdown* message."

    # an HTML-flavored message
    tgcli bot send --token "BotToken" -r "123456" --format "html" "My <strong>HTML</strong> message."

    # token as environment variable
    export TELEGRAM_BOT_TOKEN="BotToken"
    tgcli bot send -r "123456" "Hey!"
