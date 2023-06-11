#/bin/bash
cargo build -r
cp target/release/sndm ./
zip -r exp/sndm_lin.zip sndm admin.env benutzer.txt logins.txt static/
rm sndm
cargo build -r --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/sndm.exe ./
zip -r exp/sndm_win.zip sndm.exe admin.env benutzer.txt logins.txt static/
rm sndm.exe