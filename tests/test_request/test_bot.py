import requests_mock

import tgcli.request.bot


class TestBotSession:
    def setup_method(self):
        self.session = tgcli.request.bot.BotSession("0")

    def test_has_token(self):
        assert hasattr(self.session, "token")

    def test_has_base_url(self):
        assert hasattr(self.session, "base_url")


class TestBotRequest:
    def setup_method(self):
        self.session = tgcli.request.bot.BotSession("0")
        self.request = tgcli.request.bot.BotRequest(self.session, "getMe")

    def test_url(self):
        url = "{base_url}{method_name}".format(
            base_url=self.session.base_url, method_name="getMe"
        )
        assert self.request.url == url


class TestAuthenticationRequest:
    def setup_method(self):
        self.adapter = requests_mock.Adapter()

        self.session = tgcli.request.bot.BotSession("0")
        self.session._is_mocked = True

        self.request = tgcli.request.bot.AuthenticationRequest(self.session)

        self.session.mount("mock", self.adapter)

    def test_url(self):
        assert self.request.url[-5:] == "getMe"

    def test_found_status(self):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.1.json"
        ) as f:
            self.adapter.register_uri(
                "POST", self.request.url, text=f.read(), status_code=200
            )

        response = self.session.send(self.request)
        assert response.status_code == 200

    def test_found_ok(self):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.1.json"
        ) as f:
            self.adapter.register_uri(
                "POST", self.request.url, text=f.read(), status_code=200
            )

        response = self.session.send(self.request)
        assert response.json().get("ok") == True

    def test_found_result(self):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.1.json"
        ) as f:
            self.adapter.register_uri(
                "POST", self.request.url, text=f.read(), status_code=200
            )

        response = self.session.send(self.request)
        assert response.json().get("result").get("id") == 1
        assert response.json().get("result").get("is_bot") == True
        assert response.json().get("result").get("first_name") == "FooBot"
        assert response.json().get("result").get("username") == "foobot"

    def test_notfound_status(self):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.2.json"
        ) as f:
            self.adapter.register_uri(
                "POST", self.request.url, text=f.read(), status_code=404
            )

        response = self.session.send(self.request)
        assert response.status_code == 404

    def test_notfound_ok(self):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.2.json"
        ) as f:
            self.adapter.register_uri(
                "POST", self.request.url, text=f.read(), status_code=404
            )

        response = self.session.send(self.request)
        assert response.json().get("ok") == False

    def test_notfound_error_code(self):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.2.json"
        ) as f:
            self.adapter.register_uri(
                "POST", self.request.url, text=f.read(), status_code=404
            )

        response = self.session.send(self.request)
        assert response.json().get("error_code") == 404

    def test_notfound_description(self):
        with open(
            "tests/resources/responses/bot.TestAuthenticationRequest.2.json"
        ) as f:
            self.adapter.register_uri(
                "POST", self.request.url, text=f.read(), status_code=404
            )

        response = self.session.send(self.request)
        assert response.json().get("description") == "Not Found"


class TestSendMessageRequest:
    def setup_method(self):
        self.adapter = requests_mock.Adapter()

        self.session = tgcli.request.bot.BotSession("0")
        self.session._is_mocked = True

        self.request = tgcli.request.bot.SendMessageRequest(
            self.session, 1, "foo"
        )

        import json

        self.request_body = json.loads(self.request.body.decode("utf-8"))
        self.session.mount("mock", self.adapter)

    def test_url(self):
        assert self.request.url[-11:] == "sendMessage"

    def test_request_body_chat_id(self):
        assert self.request_body.get("chat_id") == 1

    def test_request_body_text(self):
        assert self.request_body.get("text") == "foo"

    def test_request_body_parse_mode(self):
        assert self.request_body.get("parse_mode") == "Markdown"


class TestSendDocumentRequest:
    def setup_method(self):
        self.adapter = requests_mock.Adapter()

        self.session = tgcli.request.bot.BotSession("0")
        self.session._is_mocked = True

        self.file = open("tests/resources/file.png", "rb")
        self.request = tgcli.request.bot.SendFileRequest(
            self.session, 1, self.file, "lorem ipsum"
        )
        self.session.mount("mock", self.adapter)

    def teardown_method(self):
        self.file.close()

    def test_default_url(self):
        assert self.request.url[-12:] == "sendDocument"

    def test_photo_url(self):
        session = tgcli.request.bot.BotSession("0")
        request = tgcli.request.bot.SendFileRequest(
            self.session,
            1,
            self.file,
            "lorem ipsum",
            tgcli.request.bot.MediaType.PHOTO,
        )
        assert request.url[-9:] == "sendPhoto"

    def test_audio_url(self):
        session = tgcli.request.bot.BotSession("0")
        request = tgcli.request.bot.SendFileRequest(
            self.session,
            1,
            self.file,
            "lorem ipsum",
            tgcli.request.bot.MediaType.AUDIO,
        )
        assert request.url[-9:] == "sendAudio"

    def test_video_url(self):
        session = tgcli.request.bot.BotSession("0")
        request = tgcli.request.bot.SendFileRequest(
            self.session,
            1,
            self.file,
            "lorem ipsum",
            tgcli.request.bot.MediaType.VIDEO,
        )
        assert request.url[-9:] == "sendVideo"

    def test_request_body_chat_id(self):
        assert b"chat_id" in self.request.body

    def test_request_body_caption(self):
        assert b"caption" in self.request.body

    def test_request_body_parse_mode(self):
        assert b"parse_mode" in self.request.body

    def test_request_body_disable_notification(self):
        assert b"disable_notification" in self.request.body

    def test_request_body_document(self):
        assert b'filename="file.png"' in self.request.body
