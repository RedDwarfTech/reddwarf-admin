#!/usr/bin/env bash

set -u
set -e
set -x

# https://stackoverflow.com/questions/71637346/option-to-get-the-rust-project-dir-in-shell-script
PROJECT_DIR="$(dirname "$(cargo locate-project|jq -r .root)")"

diesel --database-url postgres://postgres:kZLhtt5ZSN@reddwarf-postgresql.reddwarf-storage.svc.cluster.local:5432/dolphin \
migration run --config-file="${PROJECT_DIR}"/scripts/diesel/diesel-dolphin.toml
