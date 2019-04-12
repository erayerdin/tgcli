# Getting Started

## Installation

The easiest way to install `tgcli` is to use `pip`:

    pip install tgcli

## A Basic Usage

In this tutorial, our goal is to send ourselves a message using a bot.

### Preparation

#### Creating A Bot

In order to create a bot, you will need to contact
[@BotFather](https://t.me/BotFather). Steps are:

 - Send `/newbot` to `@BotFather`.
 - Give a human-readable name for your bot.
 - Give a computer-readable username for your bot.

Then `@BotFather` will provide you a token for HTTP API. Note it down to
somewhere.

Now, search your bot with the username you've just set. Send the bot an initial
message.

!!! Reason
    Bots cannot initiate conversation in Telegram. So you cannot send a message
    to somebody if they didn't send you one.

#### Getting User ID from Telegram

We need to know our own user ID for bot to send us a message.

!!! Info
    ID is *different from your human-readable name* or your username (the one
    that starts with (at) sign). It is an integer.

In order to get it, you need to send [@userinfobot](https://t.me/userinfobot)
any message and the bot will reply you with your information:

```
@foo
Id: 1234567890
First: Lorem
Last: Ipsum
```

Take note of your ID.

### Sending the Message

Now, we have (i) token of our bot and (ii) our user ID. You are pretty much
ready to go with these. Open up a terminal and...

```
tgcli bot send --token "BotToken" --receiver "1234567890" "Message"
# or better
tgcli bot send -t "BotToken" -r "1234567890" "Message"
```

You can also set `TELEGRAM_BOT_TOKEN` environment variable to protect your
bot token in the current terminal session. For more information about how to
use bot, refer to [Bot](bot.md) section.

## Getting Offline Information

You can use below to get usage information without referring to this
documentation everytime:

    tgcli --help

Or you can use any subcommand's help, as an example of `bot` subcommand:

    tgcli bot --help
