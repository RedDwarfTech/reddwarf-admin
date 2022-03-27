#!/usr/bin/env bash

set -u
set -e
set -x

CURRENT_DIR=$(pwd)
echo "$CURRENT_DIR"

diesel --database-url postgres://postgres:kZLhtt5ZSN@127.0.0.1:5432/dict \
migration run --config-file="${CURRENT_DIR}"/diesel-dict.toml

