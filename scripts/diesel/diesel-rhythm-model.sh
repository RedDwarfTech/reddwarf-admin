#!/usr/bin/env bash

set -u
set -e
set -x

# https://stackoverflow.com/questions/71637346/option-to-get-the-rust-project-dir-in-shell-script
# with the project dir
# we can execute the script in any of the project folder
PROJECT_DIR="$(dirname "$(cargo locate-project|jq -r .root)")"

# https://stackoverflow.com/questions/70410103/how-to-make-diesel-auto-generate-model
# the diesel_ext auto generate the database Jsonb to Jsonb datatype
# but the rust diesel could not handle Jsonb
# so map the jsonb to serde_json::Value
# https://stackoverflow.com/questions/72066886/the-trait-serialize-is-not-implemented-for-jsonb
diesel_ext --derive Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone \
--map "Jsonb serde_json::Value" \
--add-table-name \
--import-types "rocket::serde::Serialize" \
--import-types "serde::Deserialize" \
--import-types "crate::model::diesel::rhythm::rhythm_schema::*" \
--schema-file "${PROJECT_DIR}"/src/model/diesel/rhythm/rhythm_schema.rs --model > "${PROJECT_DIR}"/src/model/diesel/rhythm/rhythm_models.rs


# https://stackoverflow.com/questions/70410103/how-to-make-diesel-auto-generate-model
# diesel_ext --schema-file src/model/diesel/rhythm/rhythm_schema.rs --model > src/model/diesel/rhythm/rhythm_models.rs

