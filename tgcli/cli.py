import io
import platform

import sys
import colorful
import click
import yaspin
import yaspin.spinners

import tgcli.request.bot


IS_DARWIN = platform.system().lower() == "darwin"


MESSAGE_FORMATS = {"html": "HTML", "markdown": "Markdown"}


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


@bot.command()
@click.option(
    "--format",
    default="markdown",
    type=click.Choice(MESSAGE_FORMATS.keys()),
    help="Format of the message.",
)
@click.option("-f", "--file", type=click.File("rb"), help="File to send.")
@click.option(
    "-r", "--receiver", required=True, help="Receiver of the message."
)
@click.argument("message", required=True)
@click.pass_context
def send(ctx, format: str, file: io.BytesIO, receiver: str, message: str):
    session = tgcli.request.bot.BotSession(ctx.obj["token"])
    session.verify = ctx.obj["secure"]

    if file:
        request = tgcli.request.bot.SendFileRequest(
            session, receiver, file, message, MESSAGE_FORMATS[format]
        )
        file.close()
    else:
        request = tgcli.request.bot.SendMessageRequest(
            session, receiver, message, MESSAGE_FORMATS[format]
        )

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
