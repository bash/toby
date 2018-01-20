#!/bin/sh

if [ -n "$OPENSSL_VERSION" ]
then
  mv $HOME/rpmbuild/RPMS/x86_64/toby-$TRAVIS_TAG-1.x86_64.rpm \
     $HOME/rpmbuild/RPMS/x86_64/toby-$TRAVIS_TAG-1.x86_64.openssl.$OPENSSL_VERSION.rpm
fi
