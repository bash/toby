# Building from Source

## Setup

Building requires the latest nightly version of rust.
If you don't have [rustup](https://rustup.rs) installed, go ahead and install it.

Afterwards you can install the nightly version of rust:

```bash
rustup install nightly
```

## Download Source

Each [Release on GitHub](https://github.com/bash/toby/releases) comes with a tarball containing the source code with some preconfigured build settings. (The tarball has the name format `toby-VERSION.tar.gz`).

```
mkdir toby-build
cd toby-build
rustup override set nightly

wget URL_FROM_GITHUB_RELEASES_TAB
tar -xvf toby-*.tar.gz
```

## Compiling

The various paths toby uses (config, log) can be configured using the `./configure` command.

```bash
./configure --config-path /usr/local/etc/toby \
            --log-path /usr/local/var/log/toby \
            --runtime-path /usr/local/var/lib/toby
```

Finally, we can compile toby.

```
cargo build --release
```

The build produces two binaries: `toby` and `tobyd` which are both found in `./target/release/`.

## Installing

Assuming that the installation should be in `/usr/local/`, we can use `./scripts/install.sh` to move the files to the right place:

```
TOBY_TARGET=release ./scripts/install.sh
```

## Sytemd Unit

The systemd unit must be created manually (preferrably under `/usr/local/lib/systemd/system/toby.service`):

```
[Unit]
Description=Toby the friendly server bot
After=network.target
Requires=network.target

[Service]
Restart=always
ExecStart=/usr/local/bin/tobyd
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```