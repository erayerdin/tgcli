# Practical Usage

This page contains some examples about how to use `tgcli` practically.

## Getting Notifications When You Log In to a Machine

!!! warning
    This section is about Linux.

`$HOME/.profile` script is run each time you log in as a particular user in a machine. This might be useful if you want to get a notification if someone logs into your server.

Add line below to `$HOME/.profile`:

```bash
tgcli bot -t "BotToken" send -r "ReceiverID" message "🔑 Someone logged in as **$USER** on **$(date)**."
```

Now you will receive a message each time someone logs in to your server:

!!! example
    🔑 Someone logged in as **foo** on **Fri Sep 20 00:21:46 +03 2019**.
