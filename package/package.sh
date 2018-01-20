#!/bin/bash

tar -zcf toby.tar.gz ./conf ./units ./src Cargo.toml Cargo.lock

mkdir -p $HOME/rpmbuild/SOURCES
mv toby.tar.gz $HOME/rpmbuild/SOURCES

rpmbuild -bb package/package.spec
