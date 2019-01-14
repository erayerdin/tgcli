import click
import tgcli.request.bot


MESSAGE_FORMATS = {"html": "HTML", "markdown": "Markdown"}


@click.group()
def cli():
    pass


@cli.command()
@click.option(
    "-t",
    "--token",
    envvar="TELEGRAM_BOT_TOKEN",
    required=True,
    help="Token of bot. Can be provided via TELEGRAM_BOT_TOKEN environment variable.",
)
@click.options(
    "-f",
    "--format",
    default="markdown",
    type=click.Choice(MESSAGE_FORMATS.keys()),
    help="Format of the message.",
)
@click.argument("message", required=True)
def bot(token: str, format: str, message: str):
    pass
