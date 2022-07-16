#[macro_use]
extern crate dbase;

use std::convert::TryFrom;
use std::fmt::Debug;
use std::io::Cursor;

use dbase::{ErrorKind, FieldName, Reader, TableWriterBuilder};

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
