# toby

[![Build Status](https://travis-ci.org/bash/toby.svg?branch=master)](https://travis-ci.org/bash/toby)
![Bot: Friendly](https://img.shields.io/badge/bot-friendly-ff69b4.svg)

🤖 Toby the friendly server bot.

> **⚠️ Warning:** This is a work in progress. Do not use it in production. (Or expect things to go [up in flames](https://open.spotify.com/track/06t6JWrU05BxaKPtct2P2n).)

## What does it do?

Toby listens for incoming webhooks and can run pre-defined scripts for different projects. It requires that a `token` with a `secret` are passed in order to prevent that anyone can trigger jobs.

## What is it good for?

### Travis

It can update projects on your server in the `deploy` stage of a travis job. 
Examples: [bash/server-config](https://github.com/bash/server-config/blob/master/.travis.yml), [bash/mdl-app](https://github.com/bash/mdl-app/blob/master/.travis.yml).

In these examples travis uploads a package to [Digitalocean Spaces](https://www.digitalocean.com/products/spaces/) and triggers toby on my server who fetches und unpacks that package.

### More?

If you find another interesting use case [let me know](https://github.com/bash/toby/issues/new).

## Installation

There are RPM packages available from the [Releases](https://github.com/bash/toby/releases) tab on GitHub.

```sh
wget <URL_TO_RPM>
rpm -Uvh toby-<VERSION>-1.x86_64.rpm
```

## Endpoints

The HTTP API is exposed on port `8629` by default.

### `POST /v1/jobs/:project`

#### Params

| **name** | **description**                        |
| -------- | -------------------------------------- |
| token    | The token's identifier (e.g. `travis`) |
| secret   | Secret associated with token           |

This endpoint will trigger a job for the given project. It will return A `404` status if either the project isn't configured or when the access checks fail.

```sh
curl -X POST http://toby.server:8629/v1/jobs/dreams \
     -d token=travis \
     -d secret=$TOBY_SECRET
```

## Jobs

A job is triggered using the [`/v1/jobs/:project`] endpoint. Each job receives a unique id (incremental).

### Working Directory

Toby runs each job in a blank directory that is erased after the job has completed.

### Environment

Jobs inherit environment variables from the `tobyd` process.
Additional variables can be set using the [`environment` section](#the-environment-section-optional).

#### Special Environment variables

These environment variables take precedence over the variables set in the `[environment]` section.

| **name**           | **description**                             |
| ------------------ | ------------------------------------------- |
| `TOBY_JOB_ID`      | The current job id.                         |
| `TOBY_JOB_TRIGGER` | The job's trigger (`webhook` or `telegram`) |


### Logs

Toby stores logs in `/var/log/toby/jobs`. Log files have the format `<project>-<id>.log`.

## Configuration

Configuration files are written in [TOML (Tom's Obvious, Minimal Language)](https://github.com/toml-lang/toml) and hence use the file extension `toml`.

### Main config

The main configuration file is found in `/etc/toby/toby.toml`:

#### The `[listen]` section

```toml
[listen]
address = "0.0.0.0"
port = 8629
```

##### The `address` field

The address field defines the address to which the http server will bind.

##### The `port` field

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
access = ["dreams"]
```

### Projects

Each project lives in its own config file under `/etc/toby/conf.d/`.

The filename (without extension) serves as the project's identifier.

#### Example

File: `/etc/toby/conf.d/dreams.toml`:
```toml
[[scripts]]
command = ["dnf", "update", "dreams"]

[[scripts]]
command = ["systemctl", "restart", "dreams"]
```

#### The `[environment]` section (optional)

Holds additional environment variables in key, value pairs that are passed to the scripts.

```toml
[environment]
UNICORN_EMOJI="🦄"
```

#### The `[scripts]` section (required)

This section holds a list of scripts, which are executed in order.

##### The `command` field (required)

The command field specifies a command to execute.  
It must be given as an array where each argument is one element in the array.

```toml
[[scripts]]
command = ["systemctl", "restart", "dreams"]
```

##### The `allow_failure` field

> Failure is defined as either a on-zero exit status or an error with calling the command (e.g. not found, wrong permissions, etc.)

If set to `true`, failure of this command will be ignored and the deploy will resume as if the command was successful.

```toml
[[scripts]]
command = ["false"]
allow_failure = true
```
