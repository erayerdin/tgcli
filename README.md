# tgcli

[![PyPI](https://img.shields.io/pypi/v/tgcli.svg?style=flat-square&logo=python&logoColor=white)][pypi_url]
[![PyPI](https://img.shields.io/pypi/dm/tgcli.svg?style=flat-square&logo=python&logoColor=white)][pypi_url]
[![PyPI](https://img.shields.io/pypi/pyversions/tgcli.svg?style=flat-square&logo=python&logoColor=white)][pypi_url]
[![PyPI](https://img.shields.io/pypi/l/tgcli.svg?style=flat-square)][pypi_url]
[![][coogger_badge]](https://www.coogger.com/tgcli/@erayerdin/)
[![Telegram](https://img.shields.io/badge/telegram-%40erayerdin-%2332afed.svg?style=flat-square&logo=telegram&logoColor=white)](https://t.me/erayerdin)
[![Code Style](https://img.shields.io/badge/style-black-000000.svg?style=flat-square)](https://github.com/ambv/black)

![](resources/recording.gif)

[coogger_badge]: https://img.shields.io/badge/docs-coogger-1c472b?logo=image/png;base64,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

`tgcli` is a Python cli app for Telegram.

[pypi_url]: https://pypi.org/project/tgcli/

|              | Build | Coverage |
|--------------|-------|----------|
| **Master**   | [![Travis (.com) master](https://img.shields.io/travis/com/erayerdin/tgcli/master.svg?style=flat-square&logo=travis&logoColor=white)][travis_url] | [![](https://img.shields.io/coveralls/github/erayerdin/tgcli/master.svg?logo=star&logoColor=white&style=flat-square)][coveralls_url] |
| **Development** | [![Travis (.com) development](https://img.shields.io/travis/com/erayerdin/tgcli/development.svg?style=flat-square&logo=travis&logoColor=white)][travis_url] | [![](https://img.shields.io/coveralls/github/erayerdin/tgcli/development.svg?logo=star&logoColor=white&style=flat-square)][coveralls_url] |

[travis_url]: https://travis-ci.com/erayerdin/tgcli
[coveralls_url]: https://coveralls.io/github/erayerdin/tgcli

## Installing

Install via `pip`:

```bash
pip install tgcli
```

## Rationale

[A similar project](https://github.com/vysheng/tg), built on Python, was created by [@vysheng](https://github.com/vysheng), but it has not been updated since 2016 and considered abandoned. This tool *is not a fork* of the mentioned project, it is built from ground up.

## Example

For now, the use case is pretty simple. To send a message:

```python
tgcli bot --token "BotToken" send --receiver "UserID" message "Your message"
```

You don't need to expose your token as flag. If you set
`TELEGRAM_BOT_TOKEN` environment variable, you do not need to set
`--token` flag. Just set it before using `tgcli`:

```bash
export TELEGRAM_BOT_TOKEN="BotToken"
```

You can get more information by doing:

```bash
tgcli bot send --help
```

Also, this repository uses notification from a private bot, you can see the
example Travis configuration [here](.travis.yml). Private `TELEGRAM_BOT_TOKEN`
and `TELEGRAM_RECEIVER` environment variables were set.

This application serves a really small purpose for now. It might face
breaking changes in the future.

## Documentation

Documentation has an intensive amount of  information about how to
use `tgcli`. Refer to the
[documentation](https://tgcli.readthedocs.io/en/latest/).

## Donations

`tgcli` is a free (as in beer and speech) software that I have 
built in my leisure time and been maintaining it. If you like to
use it, please consider a small donation.

### How do I donate?

I accept donations in:

 - MiliBitcoin (mBTC)
 - Monero (XMR)

You can donate predefined (*almost* $1) or custom donation by 
clicking or scanning below.

| | Predefined (*Almost* $1) | How Much You Wish |
|-|--------------------------|-------------------|
| MiliBitcoin (mBTC) | [![mBTC predefined][mbtc_d1_qr]][mbtc_d1] | [![mBTC custom][mbtc_custom_qr]][mbtc_custom] |
| Monero (XMR)       | [![XMR predefined][xmr_d1_qr]][xmr_d1] | [![XMR custom][xmr_custom_qr]][xmr_custom]

[mbtc_d1]: bitcoin:bc1qpjqftgzvr2cstrn7lkfl7q84h0uq6k806cv9md?amount=0.00009732&message=For%20%22tgcli%22%20as%20my%20gratitude.&time=1568582579
[mbtc_d1_qr]: docs/img/mbtc_d1_qr.png

[mbtc_custom]: bitcoin:bc1qcywffxwa0rxsszgm07cyvsksxm3jxqj8z80ezh?message=For%20%22tgcli%22%20as%20my%20gratitude.&time=1568746229
[mbtc_custom_qr]: docs/img/mbtc_custom_qr.png

[xmr_d1]: monero://44Fs67hkoVxA9xrcLHTS4zfcYehBtzxo8LLcNWWaJ1HHT6SA6FN6aqai4QKfY7gU6TL65Pp46ov1aBy4E6jpV7ohRyfFbBr?tx_amount=0.015&tx_message=For%20%22tgcli%22%20as%20my%20gratitude.
[xmr_d1_qr]: docs/img/xmr_d1_qr.png

[xmr_custom]: monero://44Fs67hkoVxA9xrcLHTS4zfcYehBtzxo8LLcNWWaJ1HHT6SA6FN6aqai4QKfY7gU6TL65Pp46ov1aBy4E6jpV7ohRyfFbBr?tx_message=For%20%22tgcli%22%20as%20my%20gratitude.
[xmr_custom_qr]: docs/img/xmr_custom_qr.png