#!/usr/bin/env bash

set -u
set -e
set -x

# https://stackoverflow.com/questions/70410103/how-to-make-diesel-auto-generate-model
diesel_ext --derive Insertable,Queryable,Debug,Serialize,Deserialize,Default \
--add-table-name \
--import-types "rocket::serde::Serialize" \
--import-types "serde::Deserialize" \
--import-types "crate::model::diesel::dict::dict_schema::*" \
--schema-file src/model/diesel/dict/dict_schema.rs --model > src/model/diesel/dict/dict_models.rs

