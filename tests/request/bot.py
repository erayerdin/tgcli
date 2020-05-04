from pytest_httpserver import HTTPServer


class TestBotSession:
    def test_base_url(self, bot_session):
        assert bot_session.base_url.endswith("/bot0/")


class TestBotRequest:
    def test_url(self, bot_session, bot_request_factory):
        bot_request = bot_request_factory("getMe")
        assert bot_request.url.endswith("/bot0/getMe")


class TestAuthenticationRequest:
    def test_url(self, bot_authentication_request):
        assert bot_authentication_request.url.endswith("/getMe")

    def test_found_status(
        self, httpserver: HTTPServer, bot_session, bot_authentication_request
    ):
        httpserver.expect_request(bot_authentication_request.path).respond_with_json({})
        response = bot_session.send(bot_authentication_request)
        assert response.status_code == 200

    def test_notfound_status(
        self, httpserver: HTTPServer, bot_session, bot_authentication_request
    ):
        httpserver.expect_request(bot_authentication_request.path).respond_with_json(
            {}, 404
        )
        response = bot_session.send(bot_authentication_request)
        assert response.status_code == 404
