# Root

Root of `tgcli` application is itself. The root simply does nothing, which
means you have to use one of the subcommands below for further functionality:

 - [bot](bot.md)

## Global Options

Root contains some global options that you might be interested.

Short Flag | Full Flag | Required/Optional | Description
--- | --- | --- | ---
 | --secure/--no-secure | Optional | Whether to verify HTTPS certificate or not. Default is `True` except OSX[^1].

[^1]: OSX might be bundled with an older version of OpenSSL, which
      requires to point at certificate manually and causes `tgcli` to fail.
      That's why the requests in OSX is `--no-secure` by default. Easiest way
      to overcome this problem is to [update OpenSSL on your system](https://apple.stackexchange.com/a/126832). Then manually trigger `--secure` flag everytime.</small>

### Examples

    # note that this will not work
    tgcli --no-secure whatever
    # you need to set secure option after root for it to work properly
    # this will make every request non-secure on its subcommand
