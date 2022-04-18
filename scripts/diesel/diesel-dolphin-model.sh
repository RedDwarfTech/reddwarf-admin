#!/usr/bin/env bash

set -u
set -e
set -x

# https://stackoverflow.com/questions/71637346/option-to-get-the-rust-project-dir-in-shell-script
# with the project dir
# we can execute the script in any of the project folder
PROJECT_DIR="$(dirname "$(cargo locate-project|jq -r .root)")"

# https://stackoverflow.com/questions/70410103/how-to-make-diesel-auto-generate-model
diesel_ext --derive Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone \
--add-table-name \
--import-types "rocket::serde::Serialize" \
--import-types "serde::Deserialize" \
--import-types "crate::model::diesel::dolphin::dolphin_schema::*" \
--schema-file "${PROJECT_DIR}"/src/model/diesel/dolphin/dolphin_schema.rs --model > "${PROJECT_DIR}"/src/model/diesel/dolphin/dolphin_models.rs

