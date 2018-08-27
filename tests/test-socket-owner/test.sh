#!/bin/bash -e

TEST_DIR=$(cd $(dirname "$0") && pwd)

source "$TEST_DIR/../_assert.sh"

sudo docker build -t toby-test-socket-owner -f "$TEST_DIR/Dockerfile" .

CONTAINER_ID=$(docker run -d toby-test-socket-owner)

OWNER=$(sudo docker exec "$CONTAINER_ID" stat /usr/local/var/lib/toby/toby-workerd.sock -c '%U:%G')

sudo docker stop "$CONTAINER_ID" > /dev/null
sudo docker rm "$CONTAINER_ID" > /dev/null

assert_eq "toby:toby" "$OWNER"
