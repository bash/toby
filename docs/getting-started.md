# Getting Started

## Installation

There are RPM packages available from the [Releases](https://github.com/bash/toby/releases) tab on GitHub.

**Disclaimer:** These RPM packages only work with Fedora 27.
For other distributions [build toby from source](./building.md).

```sh
wget <URL_TO_RPM>
rpm -Uvh toby-<VERSION>-1.x86_64.rpm
```

The `tobyd` process can be started using `systemctl start toby`.

## Configuration

See [Configuration](./config.md) for a reference on how to set up projects and tokens.

