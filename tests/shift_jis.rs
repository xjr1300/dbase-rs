#[macro_use]
extern crate dbase;

use std::fmt::Debug;
use std::io::Cursor;
use std::{collections::HashMap, convert::TryFrom};

use dbase::{ErrorKind, FieldName, FieldValue, Reader, TableWriterBuilder};

/// To make sure it fails if you specify the wrong encoding label.
#[test]
fn shift_jis_new_with_label_with_wrong_label() {
    let result = TableWriterBuilder::new_with_label("wrong_encoding_label");
    assert!(result.is_err());
    match result.err().unwrap().kind() {
        ErrorKind::InvalidEncoding => (),
        _ => {
            assert!(false, "a ErrorKind is not a ErrorKind::InvalidEncoding");
        }
    }
}

/// To make sure it successes if you specify the correct encoding label.
#[test]
fn shift_jis_new_with_label_with_correct_label() {
    let result = TableWriterBuilder::new_with_label("shift_jis");
    assert!(result.is_ok());
}

dbase_record! {
    #[derive(Clone, Debug, PartialEq)]
    struct TestRecord {
        text: String
    }
}

#[test]
fn shift_jis_write_file_with_encoded_field_name() {
    let label = "shift_jis";
    let name = "属性1";
    let value = "吾輩は猫である。名前はまだない。";
    let field_name = FieldName::try_from(name).unwrap();

    let writer_builder = TableWriterBuilder::new_with_label(label)
        .unwrap()
        .add_character_field(field_name, 40);

    let record = TestRecord {
        text: value.to_string(),
    };

    let mut dst = Cursor::new(Vec::<u8>::new());
    let writer = writer_builder.build_with_dest(&mut dst);
    writer.write_records([&record]).unwrap();
    dst.set_position(0);

    let mut reader = Reader::new_with_label(dst, label).unwrap();
    let field = reader.fields().get(1).unwrap();
    assert_eq!(field.name(), name);
    let read_records = reader.read_as::<TestRecord>().unwrap();
    let read_record = read_records.get(0).unwrap();
    assert_eq!(read_record.text, value);
}

fn character_value(value: Option<&FieldValue>) -> Option<String> {
    match value {
        Some(value) => match value {
            FieldValue::Character(value) => match value {
                Some(value) => Some(value.clone()),
                None => None,
            },
            _ => None,
        },
        None => None,
    }
}

#[test]
fn shift_jis_read_field_names_and_values() {
    let mut data = HashMap::new();
    data.insert(0, (Some("浮雲"), Some("二葉亭四迷")));
    data.insert(1, (Some("十三夜"), Some("樋口一葉")));
    data.insert(2, (Some("金色夜叉"), Some("尾崎紅葉")));
    data.insert(3, (Some("三四郎"), Some("夏目漱石")));
    data.insert(4, (Some("羅生門"), Some("芥川龍之介")));
    data.insert(5, (Some("平家物語"), None));

    let mut reader =
        Reader::from_path_with_label("tests/data/shift_jis_field_name.dbf", "shift_jis").unwrap();
    assert_eq!(reader.fields().get(1).unwrap().name(), "書籍名");
    assert_eq!(reader.fields().get(2).unwrap().name(), "著者");
    for (index, record) in reader.iter_records().enumerate() {
        let record = record.unwrap();
        let book = character_value(record.get("書籍名"));
        let author = character_value(record.get("著者"));
        let book_and_author = data.get(&index).unwrap();
        let book = book.as_deref();
        assert_eq!(book.as_deref(), book_and_author.0);
        assert_eq!(author.as_deref(), book_and_author.1);
    }
}
