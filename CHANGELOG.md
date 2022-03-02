# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Exit codes have changed.
- Changed `--verbose` argument from global to local.
- Changed `bot --token` argument from global to local.
- Changed `bot send --receiver` argument from global to local.
- Changed `bot send --format` argument from global to local.
- Changed `bot send --silent` argument from global to local.

### Removed

- `bot send video --width` argument
- `bot send video --height` argument

## [v0.5.0-alpha.7] - 2021-12-17

### Changed

- Migrated entire project to async.
- Fixed exiting from SendOperation implementations.
- Migrated to Rust 2021 edition.
- Better error handling to clean up
- Reverted to synchronous paradigm.

## [v0.5.0-alpha.6]

### Added

- `-v` argument for verbosity

### Changed

- Non-verbose output is colorized now.

## [v0.5.0-alpha.5] - 2021-11-24

### Added

- `send --silent` global argument helps you to supress notification sound on the
  target device.

## [v0.5.0-alpha.4] - 2021-11-23

### Added

- `bot send poll` has been implemented.
- `bot send location` has been implemented.
- `bot send video` has been implemented.
- `bot send audio` has been implemented.

### Changed

- `bot send poll` now takes `question` as positional argument.
- `bot send poll` now requires at least two `-o`/`--option` argument or it is
  invalid.
- `question` argument on `bot send poll` can only take 1-300 characters.
- `-o`/`--option` on `bot send poll` can only take 1-100 characters.

### Removed

- `-q`/`--question` argument from `bot send poll`.
- `-w`/`--width` argument from `bot send video`.
- `-h`/`--height` argument from `bot send video`.

## [v0.5.0-alpha.3] - 2021-11-19

### Added

- `bot send photo` has been implemented.

## [v0.5.0-alpha.2] - 2021-11-19

### Added

- `bot send document` has been implemented.

## [v0.5.0-alpha.1] - 2021-11-18

### Added

- `bot send message` has been reimplemented.
- The same subcommands and arguments from the previous versions are inherited,
  but they throw "not implemented yet".

### Changed

- _Totally migrated to Rust_ with a base foundation that might be subject to be
  shaped further.

## [v0.4.0pre4] - 2020-05-04

### Added

- Added `-m|--multiple` option to `bot send poll`.
- Added `--anonymous/--no-anonymous` option to `bot send poll`.
- Added `--until` option to `bot send poll`.

## [v0.4.0pre3] - 2020-04-17

### Changed

- `--horizontal/-h` and `--vertical/-v` arguments for `bot send video` has been
  changed to `--width/-w` ad `--height/-h`.
- Default values for `--width/-w` and `--height/-h` arguments on
  `bot send video` has been set to `1920` and `1080`.
- Various bug fixes.

## [v0.3.1] - 2019-09-22

### Changed

- The description of `--horizontal` and `--vertical` options on `bot send video`
  subcommand has been updated to imply that these values are going to be used
  for determining the aspect ratio of the video thumbnail.
- Default values of `--horizontal` and `--vertical` options on `bot send video`
  has been set to `1`.
- Only migrated all subcommands to their packages and modules. No significant
  change is present on the software.

## [v0.4.0pre2] - 2019-09-19

### Changed

- The description of `--horizontal` and `--vertical` options on `bot send video`
  subcommand has been updated to imply that these values are going to be used
  for determining the aspect ratio of the video thumbnail.
- Default values of `--horizontal` and `--vertical` options on `bot send video`
  has been set to `1`.

## [v0.4.0pre1] - 2019-09-18

### Changed

- Only migrated all subcommands to their packages and modules. No significant
  change is present on the software.

## [v0.3.0] - 2019-04-28

### Added

- `bot send photo` subcommand
- `bot send video` subcommand
- `bot send audio` subcommand

### Changed

- `bot send file` subcommand to `bot send document`
- `bot send document` only sends _any_ type of data
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

- `poll` section to _Bot_ documentation

### Changed

- `send` is now a group type of command which has `message`, `file` and `poll`
  subcommands.
- Moved file sending functionality from `send` to `send file`
- Moved message sending functionality from `send` to `send message`
- `send` now only takes receiver chat id, other subcommands take related options
  and parameters
- Updated _Bot_ section of documentation

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

- Added `--as` argument to define the file type you sent.

## [v0.1.0b3] - 2019-01-22

### Changed

- Moved `--secure/--no-secure` argument to root of app.
- Moved `--token` argument to `bot` subcommand.

## [v0.1.0b2] - 2019-01-20

### Added

- Added `--secure/--no-secure` argument in order to verify Telegram API HTTPS
  certificate.

### Changed

- Fixed [issue](https://github.com/erayerdin/tgcli/issues/4) regarding OpenSSL
  issues on OSX.

## [v0.1.0b1] - 2019-01-19

### Changed

- Fixed a possible minor bug about possible inconsistent response from Telegram
  BOT API.

## [v0.1.0a4] - 2019-01-18

### Changed

- Updated `MANIFEST.in` for `tests_require` in `setup.py`.

## [v0.1.0a3] - 2019-01-18

### Added

- Added documentation.
- Now `bot send` command accepts `-f/--file`, which sends file.

### Changed

- `-f` argument in `bot send` now does not stand for `--format`, it stands for
  `--file`.

## [0.1.0a2] - 2019-01-15

### Added

- Added `MANIFEST.in` to setup.

## [0.1.0a1] - 2019-01-14

### Added

- Added `bot` subcommand to main
- Added `send` subcommand to `bot` command
