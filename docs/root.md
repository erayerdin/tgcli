# Root

Root of `tgcli` application is itself.

| Short Flag | Full Flag | Required/Optional     | Global/Local | Description               |
| ---------- | --------- | --------------------- | ------------ | ------------------------- |
| -v         |           | Optional and Multiple | Global       | Sets the verbosity level. |

The root simply does nothing, which means you have to use one of the subcommands below for further functionality:

 - [bot](bot.md)

## Verbosity

You can set the verbosity level multiple times to get more outputs.

```bash
# 3 level verbosity
tgcli bot send message "foo" -r 1234 -vvv
```

Each level increases the verbosity level a little bit. Below shows what tgcli shows on each level:

| Verbosity Level | Log Levels                      | Colorized | Level Prefix | Location Prefix | Location Target |
| --------------- | ------------------------------- | --------- | ------------ | --------------- | --------------- |
| 0               | Info, Warn, Error               | ✅         | ❌            | ❌               | tgcli           |
| 1               | Debug, Info, Error, Warn        | ❌         | ✅            | ❌               | tgcli           |
| 2               | Debug, Info, Error, Warn        | ❌         | ✅            | ✅               | tgcli           |
| 3               | Trace, Debug, Info, Error, Warn | ❌         | ✅            | ✅               | tgcli           |
| 4               | Trace, Debug, Info, Error, Warn | ❌         | ✅            | ✅               | All             |

### Verbosity Levels

Verbosity level represents how many times `-v` argument is used. `-vv` means verbosity level is 2 while the absense means verbosity level is 0.

### Log Levels

Each level defines has different purposes.

 - **Trace**: The values of some important variables.
 - **Debug**: Low-level logs of operation e.g. converting types.
 - **Info**: Logs targeting the end-user. This is the default in 0 verbosity level.
 - **Warn**: Logs that were needed but did not cause the program the halt due to having a default or fallback value to progress.
 - **Error**: Logs that caused the program to not be able to progress further and immediately halt.

!!! warning
    This is how levels are used by tgcli. The libraries it depends is not in our control and might use the levels for different purposes.

### Colorization

tgcli colorizes the entire line depending on the verbosity level.

The default verbosity level, info, does not have a color value, resulting in the default color of the terminal. The other levels have predefined colors by tgcli.

 - *Error* results in **bright red** color.
 - *Warn* results in **yellow** color.
 - *Debug* results in **blue** color.
 - *Trace* results in **cyan** color.

!!! warning
    The colors are disabled if the verbosity level is set above 0.

### Prefixes

Prefixes are some metadata prepended to the logging output.

```plain
[PREFIX1][PREFIX2] message
```

There are two kinds of prefixes in tgcli:

 - **Level Prefix**: It contains level name e.g. `[INFO]`.
 - **Location Prefix**: It contains where the error originates from e.g. `[tgcli::operation]`.

### Location

Not all logs originate from tgcli, some logs originate from other libraries such as reqwest, mio, want etc.
