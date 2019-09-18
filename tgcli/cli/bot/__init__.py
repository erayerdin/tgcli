import click

from tgcli.cli.bot.send import send


@click.group()
@click.option(
    "-t",
    "--token",
    envvar="TELEGRAM_BOT_TOKEN",
    required=True,
    help="""Token of bot. Can be provided via TELEGRAM_BOT_TOKEN environment
    variable.""",
)
@click.pass_context
def bot(ctx, token):
    ctx.obj["token"] = token


bot.add_command(send)
