#!/bin/sh

if [ -z "${OPENSSL_VERSION}" ]; then
  echo "No custom openssl version defined. Exiting."
  exit
fi

echo -en "travis_fold:start:openssl-install\\r"
echo "Installing openssl $OPENSSL_VERSION"

OPENSSL_DIR=$HOME/openssl/$OPENSSL_VERSION

if [ ! -f "$OPENSSL_DIR/bin/openssl" ]
then
  curl -O https://www.openssl.org/source/openssl-$OPENSSL_VERSION.tar.gz
  tar -zxf openssl-$OPENSSL_VERSION.tar.gz
  cd openssl-$OPENSSL_VERSION
  CC=musl-gcc ./Configure no-dso no-ssl2 no-ssl3 linux-x86_64 -fPIC --prefix="$OPENSSL_DIR"

  make --quiet -j$(nproc)
  make --quiet install
fi

echo -en "travis_fold:end:openssl-install\\r"
