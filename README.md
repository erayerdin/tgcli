# tgcli

[![PyPI](https://img.shields.io/pypi/v/tgcli.svg?style=flat-square&logo=python&logoColor=white)][pypi_url]
[![PyPI](https://img.shields.io/pypi/pyversions/tgcli.svg?style=flat-square&logo=python&logoColor=white)][pypi_url]
[![PyPI](https://img.shields.io/pypi/status/tgcli.svg?style=flat-square&logo=python&logoColor=white)][pypi_url]
[![PyPI](https://img.shields.io/pypi/l/tgcli.svg?style=flat-square)][pypi_url]
[![](https://img.shields.io/readthedocs/tgcli.svg?style=flat-square)](https://tgcli.readthedocs.io/en/latest/)
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
tgcli bot --token "BotToken" send --receiver "UserID" "Your message"
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

For more information, refer to the [documentation](https://tgcli.readthedocs.io/en/latest/).
