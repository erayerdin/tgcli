# Bot

Bots automate messaging in Telegram. They immitate an account / real person.
However, their username always finishes with `bot` suffix. In `tgcli`, bot
actions are executed under `tgcli bot` command. To get offline help, use:

    tgcli bot --help

## send

A subcommand of `bot` to send regular messages to any person, to get help:

    tgcli bot send --help

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
-t | --token | Required<sup>1</sup> | Token of bot.
-r | --receiver | Required | The receiver's ID. An integer.
 | --format | Optional | The format of message. Choices are `markdown` and `html`. Default is `markdown`.
-f | --file<sup>2</sup> | Optional | File to send.
 | --secure/--no-secure | Optional | Whether to verify HTTPS certificate or not. Default is `True` except OSX<sup>3</sup>.
 | message | Required | The message.

<small>**1:** You can also pass `TELEGRAM_BOT_TOKEN` environment variable to
current session of your terminal in order to protect your token from being
exposed regularly.</small>

<small>**2:** [Telegram bot API documentation](https://core.telegram.org/bots/api#senddocument)
clearly states that bots can send files up to 50 megabytes and character limit
of the message is limited to 1024 characters.</small>

<small>**3:** OSX might be bundled with an older version of OpenSSL, which
requires to point at certificate manually and causes `tgcli` to fail. That's
why the requests in OSX is `--no-secure` by default. Easiest way to overcome
this problem is to [update OpenSSL on your system](https://apple.stackexchange.com/a/126832).
Then manually trigger `--secure` flag everytime.</small>

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

    # send file
    export TELEGRAM_BOT_TOKEN="BotToken"
    tgcli bot send -r "123456" -f "any.txt" "File description"
