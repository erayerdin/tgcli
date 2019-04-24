import enum
import io
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
        parse_mode: str = "Markdown",
        disable_web_page_preview: bool = False,
        disable_notification: bool = False,
    ):
        try:
            chat_id = int(chat_id)
        except ValueError:  # pragma: no cover
            pass  # pragma: no cover

        super().__init__(session, "sendMessage")
        self.prepare_method("post")
        payload = {
            "chat_id": chat_id,
            "text": str(text),
            "parse_mode": str(parse_mode),
            "disable_web_page_preview": bool(disable_web_page_preview),
            "disable_notification": bool(disable_notification),
        }
        self.prepare_body(data=None, files=None, json=payload)


class SendPollRequest(BotRequest):
    def __init__(
        self,
        session: BotSession,
        chat_id: typing.Union[str, int],
        question: str,
        options: typing.Iterable[str],
        disable_notification: bool = False,
    ):
        try:
            chat_id = int(chat_id)
        except ValueError:  # pragma: no cover
            pass  # pragma: no cover

        super().__init__(session, "sendPoll")
        self.prepare_method("post")
        payload = {
            "chat_id": chat_id,
            "question": str(question),
            "options": tuple(options),
            "disable_notification": bool(disable_notification),
        }
        self.prepare_body(data=None, files=None, json=payload)


class SendLocationRequest(BotRequest):
    def __init__(
        self,
        session: BotSession,
        chat_id: typing.Union[str, int],
        latitude: float,
        longitude: float,
        disable_notification: bool = False,
    ):
        try:
            chat_id = int(chat_id)
        except ValueError:  # pragma: no cover
            pass  # pragma: no cover

        super().__init__(session, "sendLocation")
        self.prepare_method("post")
        payload = {
            "chat_id": chat_id,
            "latitude": float(latitude),
            "longitude": float(longitude),
            "disable_notification": bool(disable_notification),
        }
        self.prepare_body(data=None, files=None, json=payload)


@enum.unique
class MediaType(enum.Enum):
    DOCUMENT = "document"
    PHOTO = "photo"
    VIDEO = "video"
    AUDIO = "audio"


class SendFileRequest(BotRequest):
    def __init__(
        self,
        session: BotSession,
        chat_id: typing.Union[str, int],
        file: io.BytesIO,
        caption: str,
        media_type: MediaType = MediaType.DOCUMENT,
        parse_mode: str = "Markdown",
        disable_notification: bool = False,
    ):
        try:
            chat_id = int(chat_id)
        except ValueError:  # pragma: no cover
            pass  # pragma: no cover

        super().__init__(
            session, "send{}".format(str(media_type.value.title()))
        )
        payload = {
            "chat_id": chat_id,
            "caption": str(caption),
            "parse_mode": str(parse_mode),
            "disable_notification": bool(disable_notification),
        }
        self.prepare_method("post")
        self.prepare_body(
            data=payload, files={str(media_type.value): file}, json=None
        )
        self.file = file
