#/bin/bash
set -e
cargo build -r
ssh aws-web rm -r website
scp -r target/release/schiller-db static/ admin.env benutzer.txt logins.txt schiller-db.db aws-web:website