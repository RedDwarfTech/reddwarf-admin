#!/usr/bin/env bash

set -u
set -e
set -x

CURRENT_DIR=$(pwd)
echo "$CURRENT_DIR"

diesel --database-url postgres://postgres:kZLhtt5ZSN@127.0.0.1:5432/rhythm \
migration run --config-file="${CURRENT_DIR}"/diesel-rhythm.toml

