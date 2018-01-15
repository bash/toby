#!/bin/bash

tar -zcvf toby.tar.gz ./conf ./units ./src Cargo.toml Cargo.lock
cp toby.tar.gz $HOME/rpmbuild/SOURCES
rpmbuild -bb package/package.spec
