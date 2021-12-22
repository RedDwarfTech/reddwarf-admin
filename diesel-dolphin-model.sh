#!/usr/bin/env bash

set -u
set -e
set -x

# https://stackoverflow.com/questions/70410103/how-to-make-diesel-auto-generate-model
diesel_ext --derive Queryable,Debug,Serialize,Deserialize,Default \
--import-types "rocket::serde::Serialize" \
--import-types "serde::Deserialize" \
--schema-file src/model/diesel/dolphin/dolphin_schema.rs --model > src/model/diesel/dolphin/dolphin_models.rs

