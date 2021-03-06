#!/bin/sh

set -e

if [ -z "${OPENSSL_VERSION}" ]; then
  echo "No custom openssl version defined. Exiting."
  exit
fi

echo "Installing openssl $OPENSSL_VERSION"

OPENSSL_DIR=$HOME/openssl/$OPENSSL_VERSION

if [ ! -f "$OPENSSL_DIR/bin/openssl" ]
then
  wget https://www.openssl.org/source/openssl-$OPENSSL_VERSION.tar.gz
  tar -zxf openssl-$OPENSSL_VERSION.tar.gz
  cd openssl-$OPENSSL_VERSION
  ./config shared no-asm no-ssl2 no-ssl3 -fPIC --prefix="$OPENSSL_DIR"

  # modify the shlib version to a unique one to make sure the dynamic
  # linker doesn't load the system one. This isn't required for 1.1.0 at the
  # moment since our Travis builders have a diff shlib version, but it doesn't hurt
  # sed -i "s/^SHLIB_MAJOR=.*/SHLIB_MAJOR=100/" Makefile
  # sed -i "s/^SHLIB_MINOR=.*/SHLIB_MINOR=0.0/" Makefile
  # sed -i "s/^SHLIB_VERSION_NUMBER=.*/SHLIB_VERSION_NUMBER=100.0.0/" Makefile

  make --quiet depend
  make --quiet install
fi
