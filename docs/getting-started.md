# Getting Started

## Installation

`tgcli` is currently being migrated to Rust and is in its alpha stage. You can install it via Snap.

[![Snap Store](https://raw.githubusercontent.com/snapcore/snap-store-badges/master/EN/%5BEN%5D-snap-store-white.svg)](https://snapcraft.io/tgcli)

However, since it is in alpha stage, the installation differs from a normal Snap installation.

```bash
sudo snap install tgcli --edge --devmode
```

Also, again, due to being in alpha stage, it does not update automatically like a regular Snap package does. In order to get the latest version, you have to reinstall it as above.

## A Basic Usage

In this tutorial, our goal is to send ourselves a message using a bot.

### Preparation

#### Creating A Bot

In order to create a bot, you will need to contact [@BotFather](https://t.me/BotFather). Steps are:

 - Send `/newbot` to `@BotFather`.
 - Give a human-readable name for your bot.
 - Give a computer-readable username for your bot.

Then `@BotFather` will provide you a token for HTTP API. Note it down to somewhere.

Now, search your bot with the username you've just set. Send the bot an initial message.

!!! Reason
    Bots cannot initiate conversation in Telegram. So you cannot send a message
    to somebody if they didn't send you one. That's why we first need to send
    a message to the bot we've just created.

#### Getting User ID from Telegram

We need to know our own user ID for bot to send us a message.

!!! Warning
    ID is *different from your human-readable name* or your username (the one
    that starts with (at) sign). It is an integer.

In order to get it, you need to send [@userinfobot](https://t.me/userinfobot) any message and the bot will reply you with your information:

```
@foo
Id: 1234567890
First: Lorem
Last: Ipsum
```

Take note of your ID.

### Sending the Message

Now, we have (i) token of our bot and (ii) our user ID. You are pretty much ready to go with these. Open up a terminal and...

```bash
tgcli bot --token "BotToken" send --receiver "1234567890" message "Message"
# or better
tgcli bot -t "BotToken" send -r "1234567890" message "Message"
```

You can also set `TELEGRAM_BOT_TOKEN` environment variable to protect your bot token in the current terminal session.

```bash
TELEGRAM_BOT_TOKEN="BotToken" tgcli bot send -r "1234567890" message "Message"
```

For more information about how to use bot, refer to [Bot](bot.md) section.

## Getting Offline Information

You can use below to get usage information without referring to this
documentation everytime:

```bash
tgcli --help
```

Or you can use any subcommand's help, as an example of `bot` subcommand:

```bash
tgcli bot --help
```
