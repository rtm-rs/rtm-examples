#!/usr/bin/env bash

## Generate entities if migrations change
#
redo-ifchange migrations/src/mod.rs

# NOTE:
#       The seq-orm-cli commands need to be run before the cargo new - in case
#       the workspace members list has been populated.  Otherwise
#       `cargo new --lib` will produce an error "failed to load manifest for
#       workspace member ..."
if ! command -v sea-orm-cli &> /dev/null
then
    echo "sea-orm-cli is missing. To install: cargo install sea-orm-cli"
    exit 1
fi

if ! command -v refinery &> /dev/null
then
    echo "refinery is missing. Install from https://github.com/rust-db/refinery/releases"
    exit 1
fi

if [[ -z "${DATABASE_URL}" ]]; then
  echo "DATABASE_URL is not set. Exiting."
  echo "Supported databases are MySQL, PostgreSQL, SQLite."
  echo "URI formats are:"
  echo "  - MySQL (See MySQL docs): protocol://[host][/database][?properties]"
  echo "  - PostgreSQL: postgres[ql]://[user[:password]@][host][:port][/dbname][?user=....&password=...]"
  echo "  - Sqlite: file://[host]/[path]/[name].db"
  echo "NOTE:"
  echo "Setting DATABASE_SCHEMA is optional for PostgreSQL (default is public)."
  echo "For MySQL & SQLite, this environment variable is ignored."
  exit 1
fi

cargo new --lib mtr
