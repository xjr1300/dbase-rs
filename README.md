# dbase-rs

Rust library to read and write .dbf (dBase / FoxPro) files.

Most of the dBase III and FoxPro types can be read and written,
with the exception of the Memo which can only be read
(writing will come in a later release).

If dbase-rs fails to read or write or does something incorrectly, don't hesitate to open an issue.

# to write a dbase file with encoding

This fork uses a [encoding_rs](https://crates.io/crates/encoding_rs) create
