#/bin/bash
rm -r ./build
set -e
bun run build
cross build -r --target x86_64-unknown-linux-gnu
ssh aws rm -r website
scp -r target/x86_64-unknown-linux-gnu/release/schiller-db build/ admin.env benutzer.txt logins.txt schiller-db.db aws:website