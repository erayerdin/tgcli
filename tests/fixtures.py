import io
import json
import typing

import tgcli.request.bot

import pytest
import requests_mock


@pytest.fixture
def mock_adapter() -> requests_mock.Adapter:
    """
    Returns a mock adapter.
    """
    return requests_mock.Adapter()


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
def bot_authentication_request(
    bot_session
) -> tgcli.request.bot.AuthenticationRequest:
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
def bot_send_message_request(
    bot_session
) -> tgcli.request.bot.SendMessageRequest:
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
    return tgcli.request.bot.SendPollRequest(
        bot_session, 1, "Foo?", ("Bar", "Baz")
    )


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
    bot_session, file_factory
) -> tgcli.request.bot.SendFileRequest:
    """
    Returns a bot send document request.

    Fixtures
    --------
    bot_session
    file_factory

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    file = file object to "tests/resources/file.png"
    thumbnail = file object to "tests/resources/file.png"
    caption = "lorem ipsum"
    """
    return tgcli.request.bot.SendDocumentRequest(
        bot_session,
        1,
        file_factory("tests/resources/file.png"),
        file_factory("tests/resources/file.png"),
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
    bot_session, file_factory
) -> tgcli.request.bot.SendAudioRequest:
    """
    Returns a bot send audio request.

    Fixtures
    --------
    bot_session
    file_factory

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    audio = file object to "tests/resources/file.png"
    caption = "lorem ipsum"
    duration = None
    performer = "Slipknot"
    title = "People=Shit"
    thumbnail = file object to "tests/resources/file.png"
    """
    return tgcli.request.bot.SendAudioRequest(
        bot_session,
        1,
        file_factory("tests/resources/file.png"),
        file_factory("tests/resources/file.png"),
        "lorem ipsum",
        performer="Slipknot",
        title="People=Shit",
    )


@pytest.fixture
def bot_send_video_request(
    bot_session, file_factory
) -> tgcli.request.bot.SendVideoRequest:
    """
    Returns a bot send video request.

    Fixtures
    --------
    bot_session
    file_factory

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    video = file object to "tests/resources/file.png"
    thumbnail = file object to "tests/resources/file.png"
    caption = "lorem ipsum"
    duration = None
    width = 640
    height = 480
    """
    return tgcli.request.bot.SendVideoRequest(
        bot_session,
        1,
        file_factory("tests/resources/file.png"),
        file_factory("tests/resources/file.png"),
        "lorem ipsum",
        width=640,
        height=480,
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
