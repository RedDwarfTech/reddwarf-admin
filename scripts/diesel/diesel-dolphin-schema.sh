#!/usr/bin/env bash

set -u
set -e
set -x

diesel --database-url postgres://postgres:kZLhtt5ZSN@127.0.0.1:5432/dolphin \
migration run --config-file=./diesel-dolphin.toml

