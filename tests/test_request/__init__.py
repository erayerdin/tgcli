class BaseSendFileTest:
    fixture_name = None

    def get_fixture(self, request):
        return request.getfixturevalue(self.fixture_name)

    def test_request_body_chat_id(self, request):
        fixture = self.get_fixture(request)
        assert b"chat_id" in fixture.body

    def test_request_body_caption(self, request):
        fixture = self.get_fixture(request)
        assert b"caption" in fixture.body

    def test_request_body_parse_mode(self, request):
        fixture = self.get_fixture(request)
        assert b"parse_mode" in fixture.body

    def test_request_body_disable_notification(self, request):
        fixture = self.get_fixture(request)
        assert b"disable_notification" in fixture.body

    def test_request_body_document(self, request):
        fixture = self.get_fixture(request)
        assert b'filename="file.png"' in fixture.body
