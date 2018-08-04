#!/bin/sh

set -e

if [ -z "$TOBY_TARGET" ]
then
  echo "Environment variable TOBY_TARGET is not set."
  echo ""
  echo "Example usage:"
  echo "TOBY_TARGET=release $0 $@"
  exit 1
fi

mkdir -p $TOBY_PREFIX/usr/local/bin/
cp ./target/$TOBY_TARGET/toby ./target/$TOBY_TARGET/tobyd $TOBY_PREFIX/usr/local/bin/

mkdir -p $TOBY_PREFIX/usr/local/etc/toby/conf.d
mkdir -p $TOBY_PREFIX/usr/local/etc/toby/scripts.d
cp ./conf/*.toml $TOBY_PREFIX/usr/local/etc/toby/