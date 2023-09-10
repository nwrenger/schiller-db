#/bin/bash
rm -r ./build
set -e
bun run build
cargo build -r
ssh aws rm -r website
scp -r target/release/schiller-db build/ admin.env benutzer.txt logins.txt schiller-db.db aws:website