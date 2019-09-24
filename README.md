# tgcli

[![PyPI](https://img.shields.io/pypi/v/tgcli.svg?style=flat-square&logo=python&logoColor=white)][pypi_url]
[![PyPI](https://img.shields.io/pypi/dm/tgcli.svg?style=flat-square&logo=python&logoColor=white)][pypi_url]
[![PyPI](https://img.shields.io/pypi/pyversions/tgcli.svg?style=flat-square&logo=python&logoColor=white)][pypi_url]
[![PyPI](https://img.shields.io/pypi/l/tgcli.svg?style=flat-square)][pypi_url]
[![](https://img.shields.io/badge/docs-coogger-1c472b)](https://www.coogger.com/tgcli/@erayerdin/)
[![Telegram](https://img.shields.io/badge/telegram-%40erayerdin-%2332afed.svg?style=flat-square&logo=telegram&logoColor=white)](https://t.me/erayerdin)
[![Code Style](https://img.shields.io/badge/style-black-000000.svg?style=flat-square)](https://github.com/ambv/black)

![](resources/recording.gif)

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