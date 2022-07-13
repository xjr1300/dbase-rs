# dbase-rs

Rust library to read and write .dbf (dBase / FoxPro) files.

Most of the dBase III and FoxPro types can be read and written,
with the exception of the Memo which can only be read
(writing will come in a later release).

If dbase-rs fails to read or write or does something incorrectly, don't hesitate to open an issue.

# to write a dbase file with encoding

* `TableWriterBuilder`が書き出しエンコーディングを管理する。
* `TableWriterBuilder.from_reader`メソッドは、`Reader`から書き出しエンコーディングを取得する。
* `TableWrierBuilder`に書き出しエンコーディングを設定するメソッドを追加する。
* `TableWriterBuilder`に`from_table_info_with_label`メソッドを追加して、既存の`from_table_info`メソッドに書き出しエンコーディングを指定する、追加の`label`引数を受け取るようにする。
* `TableWriterBuilder`の下記メソッドに対して、書き出しエンコーディングを指定するメソッドを追加する。
  * `new`
  * `build_with_dest`
  * `build_with_file_dest`
* `FieldWriter`に書き出しエンコーディングを渡す。
* `FieldValue::Character`が`len`メソッドで値をエンコードしたときのバイト数を返却する。
* `TableWriter.write_header`メソッドでフィールド名をエンコードしたときのバイト列を出力するようにする。
