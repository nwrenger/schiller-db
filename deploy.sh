#/bin/bash
set -e
npm run build
cargo build -r
ssh aws rm -r website
scp -r target/release/schiller-db static/ admin.env benutzer.txt logins.txt schiller-db.db aws-web:website