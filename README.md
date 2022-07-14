# dbase-rs

Rust library to read and write .dbf (dBase / FoxPro) files.

Most of the dBase III and FoxPro types can be read and written,
with the exception of the Memo which can only be read
(writing will come in a later release).

If dbase-rs fails to read or write or does something incorrectly, don't hesitate to open an issue.

# to write a dbase file with encoding

# test for encoding

* フィールドの名前がShift-JISで出力されるか確認するテスト
* 文字列フィールドの値がShift-JISで出力されるか確認するテスト
* フィールドの名前をShift-JISでエンコードしたとき、そのバイト数が10バイトより多い場合に、エラーとなるか確認するテスト
* フィールドの値をShift-JISでエンコードしたとき、そのバイト数がフィールドの長さより多い場合に、エラーとなるか確認するテスト
