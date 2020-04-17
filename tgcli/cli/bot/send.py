import io
import sys
import typing

import click
import colorful
import yaspin
import yaspin.spinners

import tgcli.request.bot

MESSAGE_FORMATS = {"html": "HTML", "markdown": "Markdown"}


def send_message(
    session: tgcli.request.bot.BotSession, request: tgcli.request.bot.BotRequest
):
    """
    Sends message using Yaspin.
    """
    with yaspin.yaspin(yaspin.spinners.Spinners.clock) as spinner:
        spinner.text = "Sending message..."
        response = session.send(request)

        if response.ok:
            spinner.text = "Message was sent successfully."
            spinner.ok("✔️ ")
        else:
            data = response.json()

            code = data.get("error_code", "Unknown")
            description = data.get("description", "No description found.")
            spinner.write(
                "{c.bold_red}Error Code:{c.reset} {}".format(code, c=colorful)
            )
            spinner.write(
                "{c.bold_red}Error Details:{c.reset} {}".format(description, c=colorful)
            )

            spinner.text = "Failed sending message."
            spinner.fail("❌")
            sys.exit(1)


@click.group()
@click.option("-r", "--receiver", required=True, help="Receiver of the message.")
@click.option(
    "--format",
    "format_",
    default="markdown",
    type=click.Choice(MESSAGE_FORMATS.keys()),
    help='Format of the message. Default is "markdown".',
)
@click.pass_context
def send(ctx, receiver: str, format_: str):
    ctx.obj["receiver"] = receiver
    ctx.obj["format"] = format_


@send.command()
@click.argument("message", required=True)
@click.pass_context
def message(ctx, message: str):
    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]
    receiver = ctx.obj["receiver"]
    format = ctx.obj["format"]

    request = tgcli.request.bot.SendMessageRequest(
        session, receiver, message, MESSAGE_FORMATS[format]
    )

    send_message(session, request)


@send.command()
@click.option(
    "-o",
    "--option",
    "options",
    multiple=True,
    help="An option for poll question. You can define multiple option.",
)
@click.argument("question", required=True)
@click.pass_context
def poll(ctx, options: typing.Tuple[str], question: str):
    if len(options) < 2:
        error_messages = (
            "You need to provide at least two options.",
            "You have provided only one option."
            if len(options) == 1
            else "You have provided no options.",
        )

        raise click.BadParameter(" ".join(error_messages))

    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]
    receiver = ctx.obj["receiver"]

    request = tgcli.request.bot.SendPollRequest(session, receiver, question, options)

    send_message(session, request)


@send.command()
@click.option(
    "-x", "--latitude", type=click.FLOAT, required=True, help="Latitude of location.",
)
@click.option(
    "-y", "--longitude", type=click.FLOAT, required=True, help="Longitude of location.",
)
@click.pass_context
def location(ctx, latitude: float, longitude: float):
    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]
    receiver = ctx.obj["receiver"]

    request = tgcli.request.bot.SendLocationRequest(
        session, receiver, latitude, longitude
    )

    send_message(session, request)


THUMBNAIL_OPTION = click.option("--thumbnail", type=click.File("rb"))
FILE_ARGUMENT = click.argument("file", type=click.File("rb"), required=True)


@send.command()
@click.option("-m", "--message", default="", help="The message to inline with file.")
@THUMBNAIL_OPTION
@FILE_ARGUMENT
@click.pass_context
def document(ctx, message: str, thumbnail: io.BytesIO, file: io.BytesIO):
    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]
    receiver = ctx.obj["receiver"]
    format = ctx.obj["format"]

    request = tgcli.request.bot.SendDocumentRequest(
        session, receiver, file, thumbnail, message, MESSAGE_FORMATS[format]
    )
    file.close()

    send_message(session, request)


@send.command()
@click.option("-m", "--message", default="", help="The message to inline with file.")
@FILE_ARGUMENT
@click.pass_context
def photo(ctx, message: str, file: io.BytesIO):
    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]
    receiver = ctx.obj["receiver"]
    format = ctx.obj["format"]

    request = tgcli.request.bot.SendPhotoRequest(
        session, receiver, file, message, MESSAGE_FORMATS[format]
    )
    file.close()

    send_message(session, request)


@send.command()
@click.option("-m", "--message", default="", help="The message to inline with file.")
@click.option(
    "-w",
    "--width",
    type=click.INT,
    default=1920,
    help="The width of the video. It does not affect the video itself, "
    "just affects the shape of video container in the application. Defaults to 1920.",
)
@click.option(
    "-h",
    "--height",
    type=click.INT,
    default=1080,
    help="The height of the video. It does not affect the video itself, "
    "just affects the shape of video container in the application. Defaults to 1080.",
)
@FILE_ARGUMENT
@click.pass_context
def video(ctx, message: str, width: int, height: int, file: io.BytesIO):
    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]
    receiver = ctx.obj["receiver"]
    format = ctx.obj["format"]

    request = tgcli.request.bot.SendVideoRequest(
        session,
        receiver,
        file,
        None,
        message,
        None,
        width,
        height,
        MESSAGE_FORMATS[format],
    )
    file.close()

    send_message(session, request)


@send.command()
@click.option("-m", "--message", default="", help="The message to inline with file.")
@click.option("--performer", help="The performer of audio.")
@click.option("--title", help="The title of audio.")
@FILE_ARGUMENT
@click.pass_context
def audio(ctx, message: str, performer: str, title: str, file: io.BytesIO):
    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]
    receiver = ctx.obj["receiver"]
    format = ctx.obj["format"]

    request = tgcli.request.bot.SendAudioRequest(
        session,
        receiver,
        file,
        None,
        message,
        None,
        performer,
        title,
        MESSAGE_FORMATS[format],
    )
    file.close()

    send_message(session, request)
