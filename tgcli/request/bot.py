import typing
import requests


class BotSession(requests.Session):
    def __init__(self, token: str):
        super().__init__()
        self.token = str(token)
        self._is_mocked = False

    @property
    def base_url(self):
        if self._is_mocked:
            scheme = "mock"
        else:
            scheme = "https"

        return "{scheme}://api.telegram.org/bot{token}/".format(
            scheme=scheme, token=self.token
        )


class BotRequest(requests.PreparedRequest):
    def __init__(self, session: BotSession, method_name: str):
        super().__init__()
        self.__session = session
        self.__method_name = method_name
        self.prepare()

    def prepare_url(self, url, params):
        super(BotRequest, self).prepare_url(
            "{base_url}{method_name}".format(
                base_url=self.__session.base_url,
                method_name=self.__method_name,
            ),
            params,
        )


class AuthenticationRequest(BotRequest):
    def __init__(self, session: BotSession):
        super().__init__(session, "getMe")
        self.prepare_method("post")


class SendMessageRequest(BotRequest):
    def __init__(
        self,
        session: BotSession,
        chat_id: typing.Union[str, int],
        text: str,
        parse_mode: bool = "Markdown",
        disable_web_page_preview: bool = False,
        disable_notification: bool = False,
    ):
        super().__init__(session, "sendMessage")
        self.prepare_method("post")
        self.prepare_body(
            data=None,
            files=None,
            json={
                "chat_id": chat_id,
                "text": str(text),
                "parse_mode": str(parse_mode),
                "disable_web_page_preview": bool(disable_web_page_preview),
                "disable_notification": bool(disable_notification),
            },
        )
