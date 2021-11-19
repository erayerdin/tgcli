# tgcli

![Latest Stable Release][stable_version_badge]
![Latest Release][latest_version_badge]
![Github Release Download Count][github_download_counter_badge]
![License][license_badge]
[![Docs Status][badge_docs]][docs_url]

![](resources/recording.gif)

`tgcli` is a terminal app to send messages with bots on Telegram.

[docs_url]: https://tgcli.readthedocs.io

[stable_version_badge]: https://img.shields.io/github/v/release/erayerdin/tgcli?label=stable&style=flat-square
[latest_version_badge]: https://img.shields.io/github/v/release/erayerdin/tgcli?include_prereleases&label=latest&style=flat-square
[github_download_counter_badge]: https://img.shields.io/github/downloads/erayerdin/tgcli/total?logo=github&style=flat-square
[license_badge]: https://img.shields.io/badge/license-Apache--2.0-lightgrey?style=flat-square
[badge_docs]: https://img.shields.io/readthedocs/tgcli?style=flat-square


|         | Master                                                                                                                                  | Development                                                                                                                                                 | Format                         | Coverage                           |
| ------- | --------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ | ---------------------------------- |
| Linux   | ![Linux Build Status on Master][linux_build_status_master_badge] ![Linux Test Status on Master][linux_test_status_master_badge]         | ![Linux Build Status on Development][linux_build_status_development_badge] ![Linux Test Status on Development][linux_test_status_development_badge]         | ![Format Status][format_badge] | ![Coverage Report][coverage_badge] |
| MacOS   | ![MacOS Build Status on Master][macos_build_status_master_badge] ![MacOS Test Status on Master][macos_test_status_master_badge]         | ![MacOS Build Status on Development][macos_build_status_development_badge] ![MacOS Test Status on Development][macos_test_status_development_badge]         |                                |                                    |
| Windows | ![Windows Build Status on Master][windows_build_status_master_badge] ![Windows Test Status on Master][windows_test_status_master_badge] | ![Windows Build Status on Development][windows_build_status_development_badge] ![Windows Test Status on Development][windows_test_status_development_badge] |                                |                                    |

The details of the build status can be seen [here][build_url]. The details of the coverage report can be seen [here][coverage_url].

[build_url]: https://github.com/erayerdin/tgcli/actions
[coverage_url]: https://coveralls.io/github/erayerdin/tgcli

<!-- Linux Badges -->

[linux_build_status_master_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_linux/master?logo=linux&logoColor=white&style=flat-square
[linux_build_status_development_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_linux/development?logo=linux&logoColor=white&style=flat-square

[linux_test_status_master_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_linux/master?label=test&logo=linux&logoColor=white&style=flat-square
[linux_test_status_development_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_linux/development?label=test&logo=linux&logoColor=white&style=flat-square

<!-- MacOS Badges -->

[macos_build_status_master_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_macos/master?logo=apple&logoColor=white&style=flat-square
[macos_build_status_development_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_macos/development?logo=apple&logoColor=white&style=flat-square

[macos_test_status_master_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_macos/master?label=test&logo=apple&logoColor=white&style=flat-square
[macos_test_status_development_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_macos/development?label=test&logo=apple&logoColor=white&style=flat-square

<!-- Windows Badges -->

[windows_build_status_master_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_windows/master?logo=windows&logoColor=white&style=flat-square
[windows_build_status_development_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_windows/development?logo=windows&logoColor=white&style=flat-square

[windows_test_status_master_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_windows/master?label=test&logo=windows&logoColor=white&style=flat-square
[windows_test_status_development_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/build_windows/development?label=test&logo=windows&logoColor=white&style=flat-square

<!-- Miscellaneous Badges -->

[format_badge]: https://img.shields.io/github/workflow/status/erayerdin/tgcli/format/master?label=format&logo=linux&logoColor=white&style=flat-square
[coverage_badge]: https://img.shields.io/coveralls/github/erayerdin/tgcli/master?logo=linux&logoColor=white&style=flat-square

## Installing

### Github Releases

Currently, the project is being migrated to Rust. Until a stable release, the binaries will be distributed on [Releases page][release_page_url].

[release_page_url]: https://github.com/erayerdin/tgcli/releases

### Install with [Guix](https://guix.gnu.org)

tgcli is [available](https://guix.gnu.org/en/packages/tgcli-0.3.1/) in official channel of Guix.

```bash
guix install tgcli
```

## Rationale

[A similar project](https://github.com/vysheng/tg), built on Python, was created by [@vysheng](https://github.com/vysheng), but it has not been updated since 2016 and considered abandoned. This tool *is not a fork* of the mentioned project, it is built from ground up **with Rust**.

## Example

For now, the use case is pretty simple. To send a message:

```bash
tgcli bot send message "Your message" --token "BotToken" --receiver "UserID"
```

You don't need to expose your token as flag. If you set `TELEGRAM_BOT_TOKEN` environment variable, you do not need to set `--token` flag. Just set it before using `tgcli`:

```bash
export TELEGRAM_BOT_TOKEN="BotToken"
```

You can get more information by doing:

```bash
tgcli bot send --help
```

This application serves a really small purpose for now. It might face breaking changes in the future.

## Documentation

Documentation has an intensive amount of  information about how to use `tgcli`. Refer to the [documentation][docs_url].
