#!/bin/sh

set -e

cp ./target/$TARGET/toby ./target/$TARGET/tobyd /usr/local/bin/

mkdir -p /usr/local/etc/toby/conf.d
cp ./conf/*.toml /usr/local/etc/toby/
