#/bin/bash
set -e
cargo build -r
ssh aws-web rm -r website
scp -r target/release/sndm static/ admin.env benutzer.txt logins.txt sndm.db aws-web:website