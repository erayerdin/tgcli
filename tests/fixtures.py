import os
import typing

import pytest
from pytest_httpserver import HTTPServer

import tgcli.request.bot
from tgcli.cli import cli as cli_root


@pytest.fixture
def cli():
    return cli_root


@pytest.fixture
def bot_session(httpserver: HTTPServer, bot_token) -> tgcli.request.bot.BotSession:
    """
    Creates a bot session. It is mounted with a mock adapter.
    """
    session = tgcli.request.bot.BotSession(bot_token)
    setattr(session.__class__, "mock_url", httpserver.url_for("/"))
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
