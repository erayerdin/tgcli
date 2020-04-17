import tgcli.request.bot


class TestBotSession:
    def test_has_token(self, bot_session):
        assert hasattr(bot_session, "token")

    def test_has_base_url(self, bot_session):
        assert hasattr(bot_session, "base_url")


class TestBotRequest:
    def setup_method(self):
        self.session = tgcli.request.bot.BotSession("0")
        self.request = tgcli.request.bot.BotRequest(self.session, "getMe")

    def test_url(self, bot_session, bot_request_factory):
        bot_request = bot_request_factory("getMe")
        url = "{base_url}{method_name}".format(
            base_url=bot_session.base_url, method_name="getMe"
        )
        assert bot_request.url == url


class TestAuthenticationRequest:
    def test_url(self, bot_authentication_request):
        assert bot_authentication_request.url[-5:] == "getMe"

    def test_found_status(self, mock_adapter, bot_session, bot_authentication_request):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.1.json"
        ) as f:
            mock_adapter.register_uri(
                "POST", bot_authentication_request.url, text=f.read(), status_code=200,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.status_code == 200

    def test_found_ok(self, mock_adapter, bot_session, bot_authentication_request):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.1.json"
        ) as f:
            mock_adapter.register_uri(
                "POST", bot_authentication_request.url, text=f.read(), status_code=200,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.json().get("ok")

    def test_found_result(self, mock_adapter, bot_session, bot_authentication_request):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.1.json"
        ) as f:
            mock_adapter.register_uri(
                "POST", bot_authentication_request.url, text=f.read(), status_code=200,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.json().get("result").get("id") == 1
        assert response.json().get("result").get("is_bot")
        assert response.json().get("result").get("first_name") == "FooBot"
        assert response.json().get("result").get("username") == "foobot"

    def test_notfound_status(
        self, mock_adapter, bot_session, bot_authentication_request
    ):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.2.json"
        ) as f:
            mock_adapter.register_uri(
                "POST", bot_authentication_request.url, text=f.read(), status_code=404,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.status_code == 404

    def test_notfound_ok(self, mock_adapter, bot_session, bot_authentication_request):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.2.json"
        ) as f:
            mock_adapter.register_uri(
                "POST", bot_authentication_request.url, text=f.read(), status_code=404,
            )

        response = bot_session.send(bot_authentication_request)
        assert not response.json().get("ok")

    def test_notfound_error_code(
        self, mock_adapter, bot_session, bot_authentication_request
    ):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.2.json"
        ) as f:
            mock_adapter.register_uri(
                "POST", bot_authentication_request.url, text=f.read(), status_code=404,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.json().get("error_code") == 404

    def test_notfound_description(
        self, mock_adapter, bot_session, bot_authentication_request
    ):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.2.json"
        ) as f:
            mock_adapter.register_uri(
                "POST", bot_authentication_request.url, text=f.read(), status_code=404,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.json().get("description") == "Not Found"
