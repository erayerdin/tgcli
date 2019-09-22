# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.3.1] - 2019-09-22
### Changed
 - The description of `--horizontal` and `--vertical` options on `bot send video`
 subcommand has been updated to imply that these values are going to be used for
 determining the aspect ratio of the video thumbnail.
 - Default values of `--horizontal` and `--vertical` options on `bot send video`
 has been set to `1`.
 - Only migrated all subcommands to their packages and modules.
 No significant change is present on the software.

## [v0.4.0pre2] - 2019-09-19
### Changed
 - The description of `--horizontal` and `--vertical` options on `bot send video`
 subcommand has been updated to imply that these values are going to be used for
 determining the aspect ratio of the video thumbnail.
 - Default values of `--horizontal` and `--vertical` options on `bot send video`
 has been set to `1`.

## [v0.4.0pre1] - 2019-09-18
### Changed
 - Only migrated all subcommands to their packages and modules.
 No significant change is present on the software.

## [v0.3.0] - 2019-04-28
### Added
 - `bot send photo` subcommand
 - `bot send video` subcommand
 - `bot send audio` subcommand

### Changed
 - `bot send file` subcommand to `bot send document`
 - `bot send document` only sends *any* type of data
 - Moved `--format` option from all subcommands to `send`

### Removed
 - `bot send file` subcommand
 - `--format` option on `message`
 - `--format` option on `poll`
 - `--format` option on `location`
 - `--format` option on `document`
 - `--format` option on `photo`
 - `--format` option on `video`
 - `--format` option on `audio`

## [v0.2.1] - 2019-04-20
### Added
 - `location` to `send` in cli

### Changed
 - Updated documentation for `location`

## [v0.2.0] - 2019-04-19
### Added
 - `poll` section to *Bot* documentation

### Changed
 - `send` is now a group type of command which has `message`, `file` and `poll`
 subcommands.
 - Moved file sending functionality from `send` to `send file`
 - Moved message sending functionality from `send` to `send message`
 - `send` now only takes receiver chat id, other subcommands take related
 options and parameters
 - Updated *Bot* section of documentation

## [v0.1.2] - 2019-04-19
### Added
 - Downloads counter to README

### Changed
 - Changes status from beta to stable

### Removed
 - Status badge from README

## [v0.1.1] - 2019-04-13
### Changed
 - Changed documentation theme to material.
 - Fixed issues on documentation.

## [v0.1.0b5] - 2019-03-14
### Changed
 - Download URL on `setup.py` now redirects to corresponding version `tar.gz`
 archive.

## [v0.1.0b4] - 2019-01-22

### Added
 - Added `--as` flag to define the file type you sent.

## [v0.1.0b3] - 2019-01-22

### Changed
 - Moved `--secure/--no-secure` flag to root of app.
 - Moved `--token` flag to `bot` subcommand.

## [v0.1.0b2] - 2019-01-20

### Added
 - Added `--secure/--no-secure` flag in order to verify Telegram API HTTPS certificate.

### Changed
 - Fixed [issue](https://github.com/erayerdin/tgcli/issues/4) regarding OpenSSL issues on OSX.

## [v0.1.0b1] - 2019-01-19

### Changed
 - Fixed a possible minor bug about possible inconsistent response from Telegram BOT API.

## [v0.1.0a4] - 2019-01-18

### Changed
 - Updated `MANIFEST.in` for `tests_require` in `setup.py`.

## [v0.1.0a3] - 2019-01-18

### Added
 - Added documentation.
 - Now `bot send` command accepts `-f/--file`, which sends file.

### Changed
 - `-f` flag in `bot send` now does not stand for `--format`, it stands for `--file`.

## [0.1.0a2] - 2019-01-15

### Added
 - Added `MANIFEST.in` to setup.

## [0.1.0a1] - 2019-01-14

### Added
 - Added `bot` subcommand to main
 - Added `send` subcommand to `bot` command
