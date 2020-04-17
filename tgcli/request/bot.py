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
                base_url=self.__session.base_url, method_name=self.__method_name,
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
    DOCUMENT = {"method_name": "sendDocument", "parameter_name": "document"}
    PHOTO = {"method_name": "sendPhoto", "parameter_name": "photo"}
    VIDEO = {"method_name": "sendVideo", "parameter_name": "video"}
    AUDIO = {"method_name": "sendAudio", "parameter_name": "audio"}


class BaseFileRequest(BotRequest):
    def __init__(
        self,
        session: BotSession,
        chat_id: typing.Union[str, int],
        file: io.BytesIO,
        caption: str,
        media_type: MediaType,
        parse_mode: str = "Markdown",
        disable_notification: bool = False,
        **kwargs
    ):
        try:
            chat_id = int(chat_id)
        except ValueError:  # pragma: no cover
            pass  # pragma: no cover

        super().__init__(session, media_type.value.get("method_name"))
        extra_files = kwargs.pop("extra_files", dict())

        payload = {
            "chat_id": chat_id,
            "caption": str(caption),
            "parse_mode": str(parse_mode),
            "disable_notification": bool(disable_notification),
        }
        payload = dict(**payload, **kwargs)
        self.prepare_method("post")
        self.prepare_body(
            data=payload,
            files={str(media_type.value.get("parameter_name")): file, **extra_files,},
            json=None,
        )
        self.file = file


class SendDocumentRequest(BaseFileRequest):
    def __init__(
        self,
        session: BotSession,
        chat_id: typing.Union[str, int],
        document: io.BytesIO,
        thumbnail: io.BytesIO = None,
        caption: str = "",
        parse_mode: str = "Markdown",
        disable_notification: bool = False,
    ):
        extra_files = {"thumb": thumbnail}
        payload = {
            "session": session,
            "chat_id": chat_id,
            "file": document,
            "caption": caption,
            "parse_mode": parse_mode,
            "disable_notification": disable_notification,
            "media_type": MediaType.DOCUMENT,
        }
        if thumbnail:
            super().__init__(**payload, **extra_files)
        else:
            super().__init__(**payload)


class SendPhotoRequest(BaseFileRequest):
    def __init__(
        self,
        session: BotSession,
        chat_id: typing.Union[str, int],
        photo: io.BytesIO,
        caption: str = "",
        parse_mode: str = "Markdown",
        disable_notification: bool = False,
    ):
        payload = {
            "session": session,
            "chat_id": chat_id,
            "file": photo,
            "caption": caption,
            "parse_mode": parse_mode,
            "disable_notification": disable_notification,
            "media_type": MediaType.PHOTO,
        }
        super().__init__(**payload)


class SendAudioRequest(BaseFileRequest):
    def __init__(
        self,
        session: BotSession,
        chat_id: typing.Union[str, int],
        audio: io.BytesIO,
        thumbnail: io.BytesIO = None,
        caption: str = "",
        duration: int = None,
        performer: str = None,
        title: str = None,
        parse_mode: str = "Markdown",
        disable_notification: bool = False,
    ):
        extra_files = {"thumb": thumbnail}
        payload = {
            "session": session,
            "chat_id": chat_id,
            "file": audio,
            "caption": caption,
            "parse_mode": parse_mode,
            "disable_notification": disable_notification,
            "media_type": MediaType.AUDIO,
            **(
                {"duration": int(duration)} if duration is not None else {}
            ),  # ref https://stackoverflow.com/a/14263905/2926992
            **({"performer": str(performer)} if performer is not None else {}),
            **({"title": str(title)} if title is not None else {}),
        }

        if thumbnail:
            super().__init__(**payload, **extra_files)
        else:
            super().__init__(**payload)


class SendVideoRequest(BaseFileRequest):
    def __init__(
        self,
        session: BotSession,
        chat_id: typing.Union[str, int],
        video: io.BytesIO,
        thumbnail: io.BytesIO = None,
        caption: str = "",
        duration: int = None,
        width: int = None,
        height: int = None,
        parse_mode: str = "Markdown",
        disable_notification: bool = False,
    ):
        extra_files = {"thumb": thumbnail}
        payload = {
            "session": session,
            "chat_id": chat_id,
            "file": video,
            "caption": caption,
            "parse_mode": parse_mode,
            "disable_notification": disable_notification,
            "media_type": MediaType.VIDEO,
            **({"duration": int(duration)} if duration is not None else {}),
            **({"width": int(width)} if width is not None else {}),
            **({"height": int(height)} if height is not None else {}),
        }

        if thumbnail:
            super().__init__(**payload, **extra_files)
        else:
            super().__init__(**payload)
