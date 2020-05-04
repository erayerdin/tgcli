import pytest
from click.testing import CliRunner
from pytest_httpserver import HTTPServer

_PARAMETERS = (
    # HTML flags
    ("html", "sendMessage", ("message", "<strong>foo</strong>")),
    ("html", "sendDocument", ("document", "tests/resources/file.png")),
    (
        "html",
        "sendDocument",
        ("document", "tests/resources/file.png", "-m", "<strong>foo</strong>"),
    ),
    (
        "html",
        "sendDocument",
        (
            "document",
            "tests/resources/file.png",
            "--thumbnail",
            "tests/resources/file.png",
        ),
    ),
    ("html", "sendVideo", ("video", "tests/resources/placeholder.mkv")),
    (
        "html",
        "sendVideo",
        ("video", "tests/resources/placeholder.mkv", "-m", "<strong>foo</strong>"),
    ),
    (
        "html",
        "sendVideo",
        ("video", "tests/resources/placeholder.mkv", "-w", "1920", "-h", "1080"),
    ),
    (
        "html",
        "sendAudio",
        ("audio", "tests/resources/placeholder.wav", "-m", "<strong>foo</strong>"),
    ),
    (
        "html",
        "sendAudio",
        (
            "audio",
            "tests/resources/placeholder.wav",
            "--title",
            "Lateralus",
            "--performer",
            "Tool",
        ),
    ),
    ("html", "sendPhoto", ("photo", "tests/resources/file.png")),
    (
        "html",
        "sendPhoto",
        ("photo", "tests/resources/file.png", "-m", "<strong>foo</strong>"),
    ),
    ("html", "sendLocation", ("location", "-x", "1", "-y", "1")),
    ("html", "sendPoll", ("poll", "Am I a ghost?", "-o", "Yes", "-o", "No")),
    (
        "html",
        "sendPoll",
        (
            "poll",
            "Which bands do you like?",
            "-o",
            "Tool",
            "-o",
            "Mor ve Ötesi",
            "-o",
            "Maximum the Hormone",
            "-m",
        ),
    ),
    (
        "html",
        "sendPoll",
        ("poll", "Foo or bar?", "-o", "foo", "-o", "bar", "--anonymous"),
    ),
    (
        "html",
        "sendPoll",
        ("poll", "Foo or bar?", "-o", "foo", "-o", "bar", "--until", "60"),
    ),
    # Markdown Flags
    ("markdown", "sendMessage", ("message", "**foo**")),
    (
        "markdown",
        "sendDocument",
        ("document", "tests/resources/file.png", "-m", "**foo**"),
    ),
    (
        "markdown",
        "sendVideo",
        ("video", "tests/resources/placeholder.mkv", "-m", "**foo**"),
    ),
    (
        "markdown",
        "sendAudio",
        ("audio", "tests/resources/placeholder.wav", "-m", "**foo**"),
    ),
    ("markdown", "sendPhoto", ("photo", "tests/resources/file.png")),
    ("html", "sendPhoto", ("photo", "tests/resources/file.png", "-m", "**foo**"),),
)


@pytest.mark.parametrize(
    "format_,endpoint,flags", _PARAMETERS,
)
def test_success(
    cli_runner: CliRunner,
    cli,
    bot_session,
    httpserver: HTTPServer,
    receiver_id,
    bot_token,
    format_,
    endpoint,
    flags,
):
    httpserver.expect_request(
        "/bot{}/{}".format(bot_token, endpoint)
    ).respond_with_json({})
    args = ("bot", "send", "-r", receiver_id, "--format", format_,) + flags
    result = cli_runner.invoke(cli, args)
    assert result.exit_code == 0
    assert "successfully" in result.stdout


@pytest.mark.parametrize("format_,endpoint,flags", _PARAMETERS)
def test_failure(
    cli_runner: CliRunner,
    cli,
    bot_session,
    httpserver: HTTPServer,
    receiver_id,
    bot_token,
    format_,
    endpoint,
    flags,
):
    error_code = 400
    description = "Mock error."

    httpserver.expect_request(
        "/bot{}/{}".format(bot_token, endpoint)
    ).respond_with_json(
        {"ok": False, "error_code": error_code, "description": description}, 400
    )
    args = ("bot", "send", "-r", receiver_id, "--format", format_) + flags

    result = cli_runner.invoke(cli, args)
    assert result.exit_code != 0
    assert "Failed" in result.stdout
    assert str(error_code) in result.stdout
    assert description in result.stdout
