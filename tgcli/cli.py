import io
import platform
import sys
import typing

import click
import colorful
import tgcli.request.bot
import yaspin
import yaspin.spinners

IS_DARWIN = platform.system().lower() == "darwin"


MESSAGE_FORMATS = {"html": "HTML", "markdown": "Markdown"}
MEDIA_TYPES = [str(v.value) for v in list(tgcli.request.bot.MediaType)]

###########
# Commons #
###########
def send_message(
    session: tgcli.request.bot.BotSession,
    request: tgcli.request.bot.BotRequest,
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
                "{c.bold_red}Error Details:{c.reset} {}".format(
                    description, c=colorful
                )
            )

            spinner.text = "Failed sending message."
            spinner.fail("❌")
            sys.exit(1)


##########
## Root ##
##########
@click.group()
@click.option(
    "--secure/--no-secure",
    default=(not IS_DARWIN),
    help='Whether to validate HTTPS requests. Default is "secure" except OSX. '
    'You have to update built-in OpenSSL and manually pass "secure" each '
    "time in OSX.",
)
@click.pass_context
def cli(ctx, secure: bool):
    ctx.obj = dict()
    ctx.obj["secure"] = secure


#######
# Bot #
#######
@cli.group()
@click.option(
    "-t",
    "--token",
    envvar="TELEGRAM_BOT_TOKEN",
    required=True,
    help="Token of bot. Can be provided via TELEGRAM_BOT_TOKEN environment variable.",
)
@click.pass_context
def bot(ctx, token):
    ctx.obj["token"] = token


# send #
@bot.group()
@click.option(
    "-r", "--receiver", required=True, help="Receiver of the message."
)
@click.pass_context
def send(ctx, receiver: str):
    ctx.obj["receiver"] = receiver


# send types
FORMAT_OPTION = click.option(
    "--format",
    default="markdown",
    type=click.Choice(MESSAGE_FORMATS.keys()),
    help='Format of the message. Default is "markdown".',
)


@send.command()
@FORMAT_OPTION
@click.argument("message", required=True)
@click.pass_context
def message(ctx, format: str, message: str):
    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]
    receiver = ctx.obj["receiver"]

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

    request = tgcli.request.bot.SendPollRequest(
        session, receiver, question, options
    )

    send_message(session, request)


@send.command()
@click.option(
    "-x",
    "--latitude",
    type=click.FLOAT,
    required=True,
    help="Latitude of location.",
)
@click.option(
    "-y",
    "--longitude",
    type=click.FLOAT,
    required=True,
    help="Longitude of location.",
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


@send.command()
@click.option(
    "-m", "--message", default="", help="The message to inline with file."
)
@THUMBNAIL_OPTION
@FORMAT_OPTION
@click.argument("file", type=click.File("rb"), required=True)
@click.pass_context
def document(
    ctx, message: str, thumbnail: io.BytesIO, format: str, file: io.BytesIO
):
    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]
    receiver = ctx.obj["receiver"]

    request = tgcli.request.bot.SendDocumentRequest(
        session, receiver, file, thumbnail, message, MESSAGE_FORMATS[format]
    )
    file.close()

    send_message(session, request)
