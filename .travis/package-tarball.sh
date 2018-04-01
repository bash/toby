#!/bin/sh

ARCHIVE=toby-$TRAVIS_TAG.tar

git archive --format=tar HEAD > $ARCHIVE

echo "TOBY_VERSION=$TRAVIS_TAG" > preconfigured
tar --append -f $ARCHIVE preconfigured

gzip $ARCHIVE
