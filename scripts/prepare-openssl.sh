#!/bin/bash

OPENSSL_GITHUB_TAG="OpenSSL_$(echo $OPENSSL_VERSION | sed 's/\./_/g')"
TMP_DIR=$(mktemp -d)

rm -rf ./openssl-include

wget https://github.com/openssl/openssl/archive/$OPENSSL_GITHUB_TAG.tar.gz \
     -O $TMP_DIR/openssl.tar.gz

tar -xf $TMP_DIR/openssl.tar.gz \
    -C $TMP_DIR

mv $TMP_DIR/openssl-$OPENSSL_GITHUB_TAG/include/openssl \
   ./openssl-include

rm -rf $TMP_DIR
