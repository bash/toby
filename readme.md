# toby

[![Build Status](https://travis-ci.org/bash/toby.svg?branch=master)](https://travis-ci.org/bash/toby)
![Bot: Friendly](https://img.shields.io/badge/bot-friendly-ff69b4.svg)

🤖 Toby the friendly server bot.

## What does it do?

Toby listens for incoming webhooks and can run pre-defined scripts for different projects. It requires that a `token` with a `secret` are passed in order to prevent that anyone can trigger jobs.

## Projects Using Toby

- [bash/server-config](https://github.com/bash/server-config/blob/master/.travis.yml)
- [MyelinAi/website](https://github.com/MyelinAI/website/blob/master/.travis.yml)
- ...
- [Add your project](https://github.com/bash/toby/edit/master/readme.md)

## Installation

There are RPM packages available from the [Releases](https://github.com/bash/toby/releases) tab on GitHub.

**Disclaimer:** These RPM packages only work with Fedora 27.

```sh
wget <URL_TO_RPM>
rpm -Uvh toby-<VERSION>-1.x86_64.rpm
```



