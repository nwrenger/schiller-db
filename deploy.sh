#/bin/bash
rm -r ./static/_app/ ./static/bootstrap/ ./static/index.html ./static/login.html
set -e
npm run build
cross build -r --target x86_64-unknown-linux-gnu
ssh aws rm -r website
scp -r target/x86_64-unknown-linux-gnu/release/schiller-db static/ admin.env benutzer.txt logins.txt schiller-db.db aws:website