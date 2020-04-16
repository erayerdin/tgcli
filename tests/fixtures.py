import io
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
def location() -> typing.Tuple[float]:
    """
    Returns an example latitude and longitude.
    """
    return (38.4219611, 27.0941414)


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
