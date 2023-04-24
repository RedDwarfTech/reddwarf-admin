#!/usr/bin/env bash

set -u
set -e
set -x

export RUSTFLAGS='-L /System/Volumes/Data/opt/homebrew/Cellar/libpq/15.2/lib' && cargo build

