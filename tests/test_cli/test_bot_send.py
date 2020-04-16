from click.testing import CliRunner

from tests.test_cli import (
    SKIPIF_BOT_RECEIVER_NOT_EXISTS,
    SKIPIF_BOT_TOKEN_NOT_EXISTS,
    SKIPIF_HAS_NOT_CONNECTED,
)


class TestMessage:
    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_html(
        self, cli_runner: CliRunner, cli, receiver_id: str, invoke_message_factory,
    ):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "--format",
            "html",
            "message",
            invoke_message_factory(self.__class__, self.test_html),
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_markdown(
        self, cli_runner: CliRunner, cli, receiver_id: str, invoke_message_factory,
    ):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "--format",
            "markdown",
            "message",
            invoke_message_factory(self.__class__, self.test_markdown),
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0


class TestDocument:
    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_vanilla(self, cli_runner: CliRunner, cli, receiver_id: str):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "document",
            "tests/resources/file.png",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_thumbnail(self, cli_runner: CliRunner, cli, receiver_id: str):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "document",
            "setup.py",
            "--thumbnail",
            "tests/resources/file.png",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0
        # todo does not send thumbnail

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_message(
        self, cli_runner: CliRunner, cli, receiver_id: str, invoke_message_factory,
    ):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "document",
            "setup.py",
            "-m",
            invoke_message_factory(self.__class__, self.test_message),
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0


class TestPhoto:
    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_vanilla(self, cli_runner: CliRunner, cli, receiver_id: str):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "photo",
            "tests/resources/file.png",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_message(
        self, cli_runner: CliRunner, cli, receiver_id: str, invoke_message_factory,
    ):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "photo",
            "tests/resources/file.png",
            "-m",
            invoke_message_factory(self.__class__, self.test_vanilla),
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0


class TestVideo:
    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_vanilla(self, cli_runner: CliRunner, cli, receiver_id: str):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "video",
            "tests/resources/placeholder.mkv",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_aspect_ratio(self, cli_runner: CliRunner, cli, receiver_id: str):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "video",
            "tests/resources/placeholder.mkv",
            "-h",
            "16",
            "-v",
            "9",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_message(
        self, cli_runner: CliRunner, cli, receiver_id: str, invoke_message_factory,
    ):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "video",
            "tests/resources/placeholder.mkv",
            "-m",
            invoke_message_factory(self.__class__, self.test_message),
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_message_aspect_ratio(
        self, cli_runner: CliRunner, cli, receiver_id: str, invoke_message_factory,
    ):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "video",
            "tests/resources/placeholder.mkv",
            "-m",
            invoke_message_factory(self.__class__, self.test_message_aspect_ratio),
            "-h",
            "16",
            "-v",
            "9",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0


class TestAudio:
    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_vanilla(self, cli_runner: CliRunner, cli, receiver_id: str):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "audio",
            "tests/resources/placeholder.wav",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_performer(self, cli_runner: CliRunner, cli, receiver_id: str):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "audio",
            "tests/resources/placeholder.wav",
            "--performer",
            "Eray Erdin",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_title(self, cli_runner: CliRunner, cli, receiver_id: str):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "audio",
            "tests/resources/placeholder.wav",
            "--title",
            "White Noise",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0

    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_message(
        self, cli_runner: CliRunner, cli, receiver_id: str, invoke_message_factory
    ):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "audio",
            "tests/resources/placeholder.wav",
            "-m",
            invoke_message_factory(self.__class__, self.test_message),
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0


# todo polls cannot be tested because it cannot be sent to private messages


class TestLocation:
    @SKIPIF_HAS_NOT_CONNECTED
    @SKIPIF_BOT_TOKEN_NOT_EXISTS
    @SKIPIF_BOT_RECEIVER_NOT_EXISTS
    def test_vanilla(self, cli_runner: CliRunner, cli, receiver_id: str):
        args = (
            "bot",
            "send",
            "-r",
            receiver_id,
            "location",
            "-x",
            "38.41273",
            "-y",
            "27.13838",
        )
        result = cli_runner.invoke(cli, args)
        assert result.exit_code == 0
