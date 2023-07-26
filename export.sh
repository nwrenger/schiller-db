#/bin/bash
cargo build -r
cp target/release/schiller-db ./
zip -r exp/schiller-db_lin.zip schiller-db admin.env benutzer.txt logins.txt static/
rm schiller-db
cargo build -r --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/schiller-db.exe ./
zip -r exp/schiller-db_win.zip schiller-db.exe admin.env benutzer.txt logins.txt static/
rm schiller-db.exe