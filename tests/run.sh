#!/bin/bash -e

DIR=$(cd $(dirname "$0") && pwd)

echo "$(tput bold)==> Running test-socket-owner$(tput sgr0)"
"$DIR/test-socket-owner/test.sh"