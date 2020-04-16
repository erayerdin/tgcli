import platform

import click

import tgcli.request.bot
from tgcli.cli.bot import bot

IS_DARWIN = platform.system().lower() == "darwin"

MEDIA_TYPES = [str(v.value) for v in list(tgcli.request.bot.MediaType)]


########
# Root #
########
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


cli.add_command(bot)
