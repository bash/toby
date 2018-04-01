# toby

[![Build Status](https://travis-ci.org/bash/toby.svg?branch=master)](https://travis-ci.org/bash/toby)
![Bot: Friendly](https://img.shields.io/badge/bot-friendly-ff69b4.svg)

🤖 Toby the friendly server bot.

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

**Disclaimer:** These RPM packages only work with Fedora 27.

```sh
wget <URL_TO_RPM>
rpm -Uvh toby-<VERSION>-1.x86_64.rpm
```

## HTTP API

The HTTP API is exposed on port `8629` by default.

### Authorization

Authorization is done throught the `Authorization` header.
It is prefixed with `Token ` and contains the token and secret separated with a colon.

```
Authorization: Token travis:0dbCv0tPHHNZ3KMLiWuPO
```

### `POST /v1/jobs/:project`

This endpoint will trigger a job for the given project. It will return `403 Forbidden` if either the project isn't configured or when the access checks fail.

```sh
curl -X POST http://toby.server:8629/v1/jobs/dreams \
     -H "Authorization: Token travis:$TOBY_SECRET"
```

## Jobs

A job is triggered using the [`/v1/jobs/:project`] endpoint. Each job receives a unique id (incremental).

### Execution Order

Jobs are executed in the same order that they were queued. Note however that this is only true for jobs of the same project.  
This will allow for future changes to run jobs for different projects in parallel.

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
