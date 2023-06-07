#/bin/bash
cargo build -r
zip -r exp/sndm_lin.zip target/release/sndm admin.env benutzer.txt static/
cargo build -r --target x86_64-pc-windows-gnu
zip -r exp/sndm_win.zip target/x86_64-pc-windows-gnu/release/sndm.exe admin.env benutzer.txt static/