import io
import json
import os
import typing

import pytest
import requests_mock

import tgcli.request.bot
from tgcli.cli import cli as cli_root


@pytest.fixture
def mock_adapter() -> requests_mock.Adapter:
    """
    Returns a mock adapter.
    """
    return requests_mock.Adapter()


@pytest.fixture
def cli():
    return cli_root


@pytest.fixture
def bot_session(mock_adapter) -> tgcli.request.bot.BotSession:
    """
    Creates a bot session. It is mounted with a mock adapter.

    Fixtures
    --------
    mock_adapter

    Attributes
    ----------
    token = "0"
    _is_mocked = True
    """
    session = tgcli.request.bot.BotSession("0")
    session.mount("mock", mock_adapter)
    session._is_mocked = True
    return session


@pytest.fixture
def bot_request_factory(bot_session) -> callable:
    """
    Returns a factory that creates a bot request.

    Fixtures
    --------
    bot_session
    """

    def factory(method_name: str) -> tgcli.request.bot.BotRequest:
        return tgcli.request.bot.BotRequest(bot_session, method_name)

    return factory


@pytest.fixture
def bot_authentication_request(bot_session) -> tgcli.request.bot.AuthenticationRequest:
    """
    Returns a bot authentication request.

    Fixtures
    --------
    bot_session

    Attributes
    ----------
    session = bot_session
    """
    return tgcli.request.bot.AuthenticationRequest(bot_session)


@pytest.fixture
def bot_send_message_request(bot_session) -> tgcli.request.bot.SendMessageRequest:
    """
    Returns a bot send message request.

    Fixtures
    --------
    bot_session

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    text = "foo"
    """
    return tgcli.request.bot.SendMessageRequest(bot_session, 1, "foo")


@pytest.fixture
def bot_send_poll_request(bot_session) -> tgcli.request.bot.SendPollRequest:
    """
    Returns a bot send poll request.

    Fixtures
    --------
    bot_session

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    question = "Foo?"
    options = ("Bar", "Baz")
    """
    return tgcli.request.bot.SendPollRequest(bot_session, 1, "Foo?", ("Bar", "Baz"))


@pytest.fixture
def location() -> typing.Tuple[float]:
    """
    Returns an example latitude and longitude.
    """
    return (38.4219611, 27.0941414)


@pytest.fixture
def bot_send_location_request(
    bot_session, location
) -> tgcli.request.bot.SendLocationRequest:
    """
    Returns a bot send location request.

    Fixtures
    --------
    bot_session
    location

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    latitude = location[0]
    longitude = location[1]
    """
    return tgcli.request.bot.SendLocationRequest(
        bot_session, 1, location[0], location[1]
    )


@pytest.fixture
def bot_send_document_request(
    request, bot_session, file_factory
) -> tgcli.request.bot.SendDocumentRequest:
    """
    Returns a bot send document request.

    Fixtures
    --------
    request
    bot_session
    file_factory

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    file = file object to "tests/resources/file.png"
    thumbnail = file object to "tests/resources/file.png" or request.param
    caption = "lorem ipsum"
    """
    thumbnail = getattr(request, "param", dict()).get(
        "thumbnail", file_factory("tests/resources/file.png")
    )
    return tgcli.request.bot.SendDocumentRequest(
        bot_session,
        1,
        file_factory("tests/resources/file.png"),
        thumbnail,
        "lorem ipsum",
    )


@pytest.fixture
def bot_send_photo_request(
    bot_session, file_factory
) -> tgcli.request.bot.SendPhotoRequest:
    """
    Returns a bot send photo request.

    Fixtures
    --------
    bot_session
    file_factory

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    photo = file object to "tests/resources/file.png"
    caption = "lorem ipsum"
    """
    return tgcli.request.bot.SendPhotoRequest(
        bot_session, 1, file_factory("tests/resources/file.png"), "lorem ipsum"
    )


@pytest.fixture
def bot_send_audio_request(
    request, bot_session, file_factory
) -> tgcli.request.bot.SendAudioRequest:
    """
    Returns a bot send audio request.

    Fixtures
    --------
    request
    bot_session
    file_factory

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    audio = file object to "tests/resources/file.png"
    caption = "lorem ipsum"
    duration = None or request.param
    performer = "Slipknot" or request.param
    title = "People=Shit" or request.param
    thumbnail = file object to "tests/resources/file.png"
    """
    thumbnail = getattr(request, "param", dict()).get(
        "thumbnail", file_factory("tests/resources/file.png")
    )
    duration = getattr(request, "param", dict()).get("duration", None)
    performer = getattr(request, "param", dict()).get("performer", "Slipknot")
    title = getattr(request, "param", dict()).get("title", "People=Shit")
    return tgcli.request.bot.SendAudioRequest(
        bot_session,
        1,
        file_factory("tests/resources/file.png"),
        thumbnail,
        "lorem ipsum",
        duration,
        performer=performer,
        title=title,
    )


@pytest.fixture
def bot_send_video_request(
    request, bot_session, file_factory
) -> tgcli.request.bot.SendVideoRequest:
    """
    Returns a bot send video request.

    Fixtures
    --------
    request
    bot_session
    file_factory

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    video = file object to "tests/resources/file.png"
    thumbnail = file object to "tests/resources/file.png"
    caption = "lorem ipsum"
    duration = None or request.param
    width = 640 or request.param
    height = 480 or request.param
    """
    thumbnail = getattr(request, "param", dict()).get(
        "thumbnail", file_factory("tests/resources/file.png")
    )
    duration = getattr(request, "param", dict()).get("duration", None)
    width = getattr(request, "param", dict()).get("width", 640)
    height = getattr(request, "param", dict()).get("height", 480)
    return tgcli.request.bot.SendVideoRequest(
        bot_session,
        1,
        file_factory("tests/resources/file.png"),
        thumbnail,
        "lorem ipsum",
        duration,
        width=width,
        height=height,
    )


@pytest.fixture
def request_body_factory() -> callable:
    """
    Returns a factory which deserializes JSON body from JSON to dict.
    """

    def factory(request: requests_mock.request.requests.Request) -> dict:
        return json.loads(request.body.decode("utf-8"))

    return factory


@pytest.fixture
def file_factory(request):
    """
    Returns a factory which opens a file in rb mode and properly closes it.

    Fixtures
    --------
    request - In order to use finalizer.
    """

    def factory(file_path: str) -> io.FileIO:
        file = open(file_path, "rb")
        request.addfinalizer(lambda: file.close())
        return file

    return factory


@pytest.fixture
def bot_token() -> typing.Union[str, None]:
    """
    Returns TELEGRAM_BOT_TOKEN environment variable.
    """
    return os.environ.get("TELEGRAM_BOT_TOKEN")


@pytest.fixture
def receiver_id() -> typing.Union[str, None]:
    """
    Returns TELEGRAM_RECEIVER environment variable.
    """
    return os.environ.get("TELEGRAM_RECEIVER")


@pytest.fixture
def invoke_message_factory():
    """
    A factory to generate message for Telegram.
    """

    def factory(klass: typing.ClassVar, method: typing.Callable):
        return "A message invoked by `{}::{}`.".format(klass.__name__, method.__name__)

    return factory
