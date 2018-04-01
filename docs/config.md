# Configuration

Configuration files are written in [TOML (Tom's Obvious, Minimal Language)](https://github.com/toml-lang/toml) and hence use the file extension `toml`.

## Main config

The main configuration file is found in `/etc/toby/toby.toml`:

### The `[listen]` section

```toml
[listen]
address = "0.0.0.0"
port = 8629
```

#### The `address` field

The address field defines the address to which the http server will bind.

#### The `port` field

The port field specifies the port on which the http server will listen.

### The `[telegram]` section

This section configures the integration with [Telegram](https://www.telegram.org).  
[There's a guide for setting up Telegram with Toby](telegram.md).

```toml
[telegram]
token = "..."
```

#### The `token` field (required)

This field contains the bot token obtained from the [BotFather](https://t.me/BotFather).

#### The `send_log` field

When to send the job's log file after the job has completed.
One of: `never`, `always`, `success`, `failure`. Defaults to `never`.  

## Tokens

The tokens used to call the webhook are found in `/etc/toby/tokens.toml`.

Each section in this file represents one token where the section name serves as the identifier.

### The `secret` field (required)

This should be a random string. It is used as an authorisation mechanism for the webhook.

The command `toby gen-secret` will generate a random (url-safe) secret for you.

```toml
[travis]
secret = "zXwFQohbdnPXIRqCDUKw7"
access = ["..."]
```

### The `access` field (required)

This field lists the project's by identifier to which this token will have access.

```toml
[travis]
secret = "..."
access = ["dreams"]
```

## Projects

Each project lives in its own config file under `/etc/toby/conf.d/`.

The filename (without extension) serves as the project's identifier.

### Example

File: `/etc/toby/conf.d/dreams.toml`:
```toml
[[scripts]]
command = ["dnf", "update", "dreams"]

[[scripts]]
command = ["systemctl", "restart", "dreams"]
```

### The `[environment]` section (optional)

Holds additional environment variables in key, value pairs that are passed to the scripts.

```toml
[environment]
UNICORN_EMOJI="🦄"
```

### The `[scripts]` section (required)

This section holds a list of scripts, which are executed in order.

#### The `command` field (required)

The command field specifies a command to execute.  
It must be given as an array where each argument is one element in the array.

```toml
[[scripts]]
command = ["systemctl", "restart", "dreams"]
```

#### The `allow_failure` field

> Failure is defined as either a on-zero exit status or an error with calling the command (e.g. not found, wrong permissions, etc.)

If set to `true`, failure of this command will be ignored and the deploy will resume as if the command was successful.

```toml
[[scripts]]
command = ["false"]
allow_failure = true
```
