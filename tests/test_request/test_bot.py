import requests_mock

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

    def test_found_status(
        self, mock_adapter, bot_session, bot_authentication_request
    ):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.1.json"
        ) as f:
            mock_adapter.register_uri(
                "POST",
                bot_authentication_request.url,
                text=f.read(),
                status_code=200,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.status_code == 200

    def test_found_ok(
        self, mock_adapter, bot_session, bot_authentication_request
    ):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.1.json"
        ) as f:
            mock_adapter.register_uri(
                "POST",
                bot_authentication_request.url,
                text=f.read(),
                status_code=200,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.json().get("ok")

    def test_found_result(
        self, mock_adapter, bot_session, bot_authentication_request
    ):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.1.json"
        ) as f:
            mock_adapter.register_uri(
                "POST",
                bot_authentication_request.url,
                text=f.read(),
                status_code=200,
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
                "POST",
                bot_authentication_request.url,
                text=f.read(),
                status_code=404,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.status_code == 404

    def test_notfound_ok(
        self, mock_adapter, bot_session, bot_authentication_request
    ):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.2.json"
        ) as f:
            mock_adapter.register_uri(
                "POST",
                bot_authentication_request.url,
                text=f.read(),
                status_code=404,
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
                "POST",
                bot_authentication_request.url,
                text=f.read(),
                status_code=404,
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
                "POST",
                bot_authentication_request.url,
                text=f.read(),
                status_code=404,
            )

        response = bot_session.send(bot_authentication_request)
        assert response.json().get("description") == "Not Found"


class TestSendMessageRequest:
    def test_url(self, bot_send_message_request):
        assert bot_send_message_request.url[-11:] == "sendMessage"

    def test_request_body_chat_id(
        self, bot_send_message_request, request_body_factory
    ):
        request_body = request_body_factory(bot_send_message_request)
        assert request_body.get("chat_id") == 1

    def test_request_body_text(
        self, bot_send_message_request, request_body_factory
    ):
        request_body = request_body_factory(bot_send_message_request)
        assert request_body.get("text") == "foo"

    def test_request_body_parse_mode(
        self, bot_send_message_request, request_body_factory
    ):
        request_body = request_body_factory(bot_send_message_request)
        assert request_body.get("parse_mode") == "Markdown"


class TestSendDocumentRequest:
    def test_url(self, bot_send_document_request):
        assert bot_send_document_request.url[-12:] == "sendDocument"

    def test_request_body_chat_id(self, bot_send_document_request):
        assert b"chat_id" in bot_send_document_request.body

    def test_request_body_caption(self, bot_send_document_request):
        assert b"caption" in bot_send_document_request.body

    def test_request_body_parse_mode(self, bot_send_document_request):
        assert b"parse_mode" in bot_send_document_request.body

    def test_request_body_disable_notification(
        self, bot_send_document_request
    ):
        assert b"disable_notification" in bot_send_document_request.body

    def test_request_body_document(self, bot_send_document_request):
        assert b'filename="file.png"' in bot_send_document_request.body


class TestSendPhotoRequest(TestSendDocumentRequest):
    def test_url(self, bot_send_document_request_factory, file_factory):
        request = bot_send_document_request_factory(
            1,
            file_factory("tests/resources/file.png"),
            "lorem ipsum",
            tgcli.request.bot.MediaType.PHOTO,
        )
        assert request.url[-9:] == "sendPhoto"


class TestSendAudioRequest(TestSendDocumentRequest):
    def test_url(self, bot_send_document_request_factory, file_factory):
        request = bot_send_document_request_factory(
            1,
            file_factory("tests/resources/file.png"),
            "lorem ipsum",
            tgcli.request.bot.MediaType.AUDIO,
        )
        assert request.url[-9:] == "sendAudio"


class TestSendVideoRequest(TestSendDocumentRequest):
    def test_url(self, bot_send_document_request_factory, file_factory):
        request = bot_send_document_request_factory(
            1,
            file_factory("tests/resources/file.png"),
            "lorem ipsum",
            tgcli.request.bot.MediaType.VIDEO,
        )
        assert request.url[-9:] == "sendVideo"


class TestSendPollRequest:
    def test_url(self, bot_send_poll_request):
        assert bot_send_poll_request.url[-8:] == "sendPoll"

    def test_request_body_chat_id(
        self, bot_send_poll_request, request_body_factory
    ):
        request_body = request_body_factory(bot_send_poll_request)
        assert request_body.get("chat_id") == 1

    def test_request_body_question(
        self, bot_send_poll_request, request_body_factory
    ):
        request_body = request_body_factory(bot_send_poll_request)
        assert request_body.get("question") == "Foo?"

    def test_request_body_options(
        self, bot_send_poll_request, request_body_factory
    ):
        request_body = request_body_factory(bot_send_poll_request)
        assert request_body.get("options")[0] == "Bar"
        assert request_body.get("options")[1] == "Baz"

    def test_request_body_disable_notification(
        self, bot_send_poll_request, request_body_factory
    ):
        request_body = request_body_factory(bot_send_poll_request)
        assert not request_body.get("disable_notification")


class TestSendLocationRequest:
    def test_url(self, bot_send_location_request):
        assert bot_send_location_request.url[-12:] == "sendLocation"

    def test_request_body_chat_id(
        self, bot_send_location_request, request_body_factory
    ):
        request_body = request_body_factory(bot_send_location_request)
        assert request_body.get("chat_id") == 1

    def test_request_body_latitude(
        self, bot_send_location_request, request_body_factory, location
    ):
        request_body = request_body_factory(bot_send_location_request)
        assert request_body.get("latitude") == location[0]

    def test_request_body_longitude(
        self, bot_send_location_request, request_body_factory, location
    ):
        request_body = request_body_factory(bot_send_location_request)
        assert request_body.get("longitude") == location[1]
