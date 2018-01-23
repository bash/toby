# toby

[![Build Status](https://travis-ci.org/bash/toby.svg?branch=master)](https://travis-ci.org/bash/toby)

🤖 Toby the friendly server bot.

> **⚠️ Warning:** This is a work in progress. Do not use it in production.

## What does it do?

Toby listens for incoming webhooks and runs a pre-defined list of scripts when it receives a webhook.

## Installation

There are RPM packages available from the [Releases](https://github.com/bash/toby/releases) tab on GitHub.

```sh
wget <URL_TO_RPM>
rpm -Uvh toby-<VERSION>-1.x86_64.rpm
```

## Endpoints

The HTTP API is exposed on port `8629` by default.

### `POST /v1/deploy/:project`

#### Params

| **name** | **description**                        |
| -------- | -------------------------------------- |
| token    | The token's identifier (e.g. `travis`) |
| secret   | Secret associated with token           |

This endpoint will trigger a deploy for the given project. It will return A `404` status if either the project isn't configured or when the access checks fail.

```sh
curl -X POST http://toby.woof:8629/v1/deploy/forest \
     -d token=travis
     -d secret=$TOBY_SECRET
```


## Configuration

Configuration files are written in [TOML (Tom's Obvious, Minimal Language)](https://github.com/toml-lang/toml) and hence use the file extension `toml`.

### Main config

The main configuration file is found in `/etc/toby/toby.toml`:

#### The `[listen]` section (required)

```toml
[listen]
address = "0.0.0.0"
port = 8629
```

##### The `address` field (required)

The address field defines the address to which the http server will bind.

##### The `port` field (required)

The port field specifies the port on which the http server will listen.

#### The `[telegram]` section

This section configures the integration with [Telegram](https://www.telegram.org).

> The integration with telegram is not finished yet and is very likely to change. See: [Issue #6](https://github.com/bash/toby/issues/6).

```toml
[telegram]
token = "..."
chat_id = "1234567"
```

##### The `token` field (required)

This field contains the bot token obtained from the [BotFather](https://t.me/BotFather).

### Tokens

The tokens used to call the webhook are found in `/etc/toby/tokens.toml`.

Each section in this file represents one token where the section name serves as the identifier.

#### The `secret` field (required)

This should be a random string. It is used as an authorisation mechanism for the webhook.

The command `toby gen-secret` will generate a random (url-safe) secret for you.

```toml
[travis]
secret = "zXwFQohbdnPXIRqCDUKw7"
access = ["..."]
```

#### The `access` field (required)

This field lists the project's by identifier to which this token will have access.

```toml
[travis]
secret = "..."
access = ["forest", "foxden"]
```

### Projects

Each project lives in its own config file under `/etc/toby/conf.d/`.

The filename (without extension) serves as the project's identifier.

#### Example

File: `/etc/toby/conf.d/playground.toml`:
```toml
[[scripts]]
command = ["dnf", "update", "playground"]

[[scripts]]
command = ["systemctl", "restart", "playground"]
```

#### The `[scripts]` section (required)

This section holds a list of scripts, which are executed in order.

##### The `command` field (required)

The command field specifies a command to execute.  
It must be given as an array where each argument is one element in the array.

```toml
[[scripts]]
command = [
  "systemctl",
  "restart",
  "playground"
]
```

##### The `allow_failure` field

> Failure is defined as either a on-zero exit status or an error with calling the command (e.g. not found, wrong permissions, etc.)

If set to true, failure of this command will be ignored and the deploy will resume as if the command was successful.

```toml
[[scripts]]
command = ["false"]
allow_failure = true
```