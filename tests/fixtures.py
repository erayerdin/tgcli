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
def bot_send_document_request_factory(bot_session) -> callable:
    """
    Returns a bot send document request factory.

    Fixtures
    --------
    bot_session
    """

    def factory(
        chat_id: int,
        file: io.FileIO,
        caption: str,
        media_type: tgcli.request.bot.MediaType,
    ) -> tgcli.request.bot.SendFileRequest:
        return tgcli.request.bot.SendFileRequest(
            bot_session, chat_id, file, caption, media_type
        )

    return factory


@pytest.fixture
def bot_send_document_request(
    bot_send_document_request_factory, file_factory
) -> tgcli.request.bot.SendFileRequest:
    """
    Returns a bot send document request.

    Fixtures
    --------
    bot_send_document_request_factory

    Attributes
    ----------
    session = bot_session
    chat_id = 1
    file = file object to "tests/resources/file.png"
    caption = "lorem ipsum"
    """
    return bot_send_document_request_factory(
        1,
        file_factory("tests/resources/file.png"),
        "lorem ipsum",
        tgcli.request.bot.MediaType.DOCUMENT,
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
