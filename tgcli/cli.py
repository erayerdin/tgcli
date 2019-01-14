import colorful
import click
import yaspin
import yaspin.spinners

import tgcli.request.bot


MESSAGE_FORMATS = {"html": "HTML", "markdown": "Markdown"}


@click.group()
def cli():
    pass


@cli.group()
def bot():
    pass


@bot.command()
@click.option(
    "-t",
    "--token",
    envvar="TELEGRAM_BOT_TOKEN",
    required=True,
    help="Token of bot. Can be provided via TELEGRAM_BOT_TOKEN environment variable.",
)
@click.option(
    "-f",
    "--format",
    default="markdown",
    type=click.Choice(MESSAGE_FORMATS.keys()),
    help="Format of the message.",
)
@click.option(
    "-r", "--receiver", required=True, help="Receiver of the message."
)
@click.argument("message", required=True)
def send(token: str, format: str, receiver: str, message: str):
    session = tgcli.request.bot.BotSession(token)
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

            code = data.get("error_code")
            description = data.get("description")
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
