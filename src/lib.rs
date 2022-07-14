//! dbase is rust library meant to read and write dBase / FoxPro files.
//!
//! Theses files are nowadays generally only found in association with shapefiles.
//!
//! # Reading
//!
//! The [Reader](struct.Reader.html) is the struct that you'll need to use in order
//! to read the content of a dBase file.
//!
//! Once you have access to the records, you will have to `match` against the real
//! [FieldValue](enum.FieldValue.html)
//!
//! ## Examples
//!
//! ```
//! use dbase::FieldValue;
//! # fn main() -> Result<(), dbase::Error> {
//! let records = dbase::read("tests/data/line.dbf")?;
//! for record in records {
//!     for (name, value) in record {
//!         println!("{} -> {:?}", name, value);
//!         match value {
//!             FieldValue::Character(Some(string)) => println!("Got string: {}", string),
//!             FieldValue::Numeric(value) => println!("Got numeric value of  {:?}", value),
//!             _ => {}
//!         }
//!     }
//!}
//! # Ok(())
//! # }
//! ```
//!
//! You can also specify a encoding.
//!
//! ```
//! use dbase::FieldValue;
//! # fn main() -> Result<(), dbase::Error> {
//! let records = dbase::read_with_label("tests/data/shift_jis.dbf", "shift_jis")?;
//! for record in records {
//!     for (name, value) in record {
//!         println!("{} -> {:?}", name, value);
//!         match value {
//!             FieldValue::Character(Some(string)) => println!("Got string: {}", string),
//!             _ => {}
//!         }
//!     }
//!}
//! # Ok(())
//! # }
//! ```
//!
//! You can also create a [Reader](reading/struct.Reader.html) and iterate over the records.
//!
//! ```
//! # fn main() -> Result<(), dbase::Error> {
//! let mut reader = dbase::Reader::from_path("tests/data/line.dbf")?;
//! for record_result in reader.iter_records() {
//!     let record = record_result?;
//!     for (name, value) in record {
//!         println!("name: {}, value: {:?}", name, value);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Deserialisation
//!
//! If you know what kind of data to expect from a particular file you can use implement
//! the [ReadableRecord](trait.ReadableRecord.html) trait to "deserialize" the record into
//! your custom struct:
//!
//! ```
//! use std::io::{Read, Seek};
//! struct StationRecord {
//!     name: String,
//!     marker_col: String,
//!     marker_sym: String,
//!     line: String,
//! }
//!
//! impl dbase::ReadableRecord for StationRecord {
//!     fn read_using<T>(field_iterator: &mut dbase::FieldIterator<T>) -> Result<Self, dbase::FieldIOError>
//!          where T: Read + Seek{
//!         Ok(Self {
//!             name: field_iterator.read_next_field_as()?.value,
//!             marker_col: field_iterator.read_next_field_as()?.value,
//!             marker_sym: field_iterator.read_next_field_as()?.value,
//!             line: field_iterator.read_next_field_as()?.value,
//!         })
//!     }
//! }
//! # fn main() -> Result<(), dbase::Error> {
//! let mut reader = dbase::Reader::from_path("tests/data/stations.dbf")?;
//! let stations = reader.read_as::<StationRecord>()?;
//!
//! // cspell:disable-next-line
//! assert_eq!(stations[0].name, "Van Dorn Street");
//! assert_eq!(stations[0].marker_col, "#0000ff");
//! assert_eq!(stations[0].marker_sym, "rail-metro");
//! assert_eq!(stations[0].line, "blue");
//! # Ok(())
//! # }
//! ```
//!
//! If you use the `serde` optional feature and serde_derive crate you can have the
//! [ReadableRecord](trait.ReadableRecord.html) implemented for you
//!
//! ```
//! # #[cfg(feature = "serde")]
//! extern crate serde_derive;
//!
//! # #[cfg(feature = "serde")]
//! # fn main() -> Result<(), dbase::Error>{
//!
//! use std::io::{Read, Seek};
//! use serde_derive::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct StationRecord {
//!     name: String,
//!     marker_col: String,
//!     marker_sym: String,
//!     line: String,
//! }
//!
//! let mut reader = dbase::Reader::from_path("tests/data/stations.dbf")?;
//! let stations = reader.read_as::<StationRecord>()?;
//!
//! // cspell:disable-next-line
//! assert_eq!(stations[0].name, "Van Dorn Street");
//! assert_eq!(stations[0].marker_col, "#0000ff");
//! assert_eq!(stations[0].marker_sym, "rail-metro");
//! assert_eq!(stations[0].line, "blue");
//! # Ok(())
//! # }
//!
//! # #[cfg(not(feature = "serde"))]
//! # fn main() {
//! # }
//! ```
//!
//!
//! # Writing
//!
//! In order to get a [TableWriter](struct.TableWriter.html) you will need to build it using
//! its [TableWriterBuilder](struct.TableWriterBuilder.html) to specify the fields that constitute
//! a record.
//!
//! As for reading, you can *serialize* structs into a dBase file, given that they match the
//! declared fields in when building the TableWriterBuilder by implementing the
//! [WritableRecord](trait.WritableRecord.html).
//!
//! ## Examples
//!
//! ```
//! # fn main() -> Result<(), dbase::Error> {
//! let mut reader = dbase::Reader::from_path("tests/data/stations.dbf")?;
//! let mut stations = reader.read()?;
//!
//! let mut writer = dbase::TableWriterBuilder::from_reader(reader)
//!     .build_with_file_dest("stations.dbf").unwrap();
//!
//! stations[0].get_mut("line").and_then(|_old| Some("Red".to_string()));
//! writer.write_records(&stations)?;
//! # Ok(())
//! # }
//! ```
//!
//! ```
//! use dbase::{TableWriterBuilder, FieldName, WritableRecord, FieldWriter, FieldIOError};
//! use std::convert::TryFrom;
//! use std::io::{Cursor, Write};
//!
//! struct User {
//!     nick_name: String,
//!     age: f64
//! }
//!
//! impl WritableRecord for User {
//!     fn write_using<'a, W: Write>(&self, field_writer: &mut FieldWriter<'a, W>) -> Result<(), FieldIOError> {
//!         field_writer.write_next_field_value(&self.nick_name)?;
//!         field_writer.write_next_field_value(&self.age)?;
//!         Ok(())
//!     }
//! }
//!
//! let mut writer = TableWriterBuilder::new()
//!     .add_character_field(FieldName::try_from("Nick Name").unwrap(), 50)
//!     .add_numeric_field(FieldName::try_from("Age").unwrap(), 20, 10)
//!     .build_with_dest(Cursor::new(Vec::<u8>::new()));
//!
//!
//! let records = User{
//!     // cspell:disable-next-line
//!     nick_name: "Yoshi".to_string(),
//!     age: 32.0,
//! };
//!
//! writer.write_record(&records);
//! ```
//!
//! If you use the serde optional feature and serde_derive crate you can have the
//! [WritableRecord](trait.WritableRecord.html) implemented for you.
//!
//! ```
//! # #[cfg(feature = "serde")]
//! extern crate serde_derive;
//!
//! # #[cfg(feature = "serde")]
//! use serde_derive::Serialize;
//!
//! use dbase::{TableWriterBuilder, FieldName, WritableRecord, FieldWriter};
//! use std::convert::TryFrom;
//! use std::io::{Cursor, Write};
//!
//! # #[cfg(feature = "serde")]
//! # fn main () {
//! #[derive(Serialize)]
//! struct User {
//!     nick_name: String,
//!     age: f64
//! }
//!
//! let writer = TableWriterBuilder::new()
//!     .add_character_field(FieldName::try_from("Nick Name").unwrap(), 50)
//!     .add_numeric_field(FieldName::try_from("Age").unwrap(), 20, 10)
//!     .build_with_dest(Cursor::new(Vec::<u8>::new()));
//!
//!
//! let records = vec![User{
//!     // cspell:disable-next-line
//!     nick_name: "Yoshi".to_string(),
//!     age: 32.0,
//! }];
//!
//!     writer.write_records(&records);
//! # }
//! # #[cfg(not(feature = "serde"))]
//! # fn main() {}
//! ```

#![deny(unstable_features)]

extern crate byteorder;
#[cfg(feature = "serde")]
extern crate serde;
extern crate time;

#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;

mod error;
mod header;
mod reading;
mod record;
mod writing;

use encoding_rs::Encoding;

pub use crate::error::{Error, ErrorKind, FieldIOError};
pub use crate::reading::{
    read, read_with_label, FieldIterator, NamedValue, ReadableRecord, Reader, Record,
    RecordIterator, TableInfo,
};
pub use crate::record::field::{Date, DateTime, FieldType, FieldValue, Time};
pub use crate::record::{FieldConversionError, FieldInfo, FieldName};
pub use crate::writing::{FieldWriter, TableWriter, TableWriterBuilder, WritableRecord};

pub(crate) fn invalid_data_error(message: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message)
}

pub(crate) fn encoded_bytes(value: &str, encoding: &'static Encoding) -> std::io::Result<Vec<u8>> {
    if encoding == encoding_rs::UTF_8 {
        Ok(value.as_bytes().to_vec())
    } else {
        let (encoded, _, result) = encoding.encode(value);
        if !result {
            return Err(invalid_data_error(format!(
                "cannot encode `{}` by `{}` encoding",
                value,
                encoding.name()
            )));
        }

        Ok(encoded.to_vec())
    }
}

/// macro to define a struct that implements the ReadableRecord and WritableRecord
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate dbase;
/// # fn main() {
/// dbase_record!(
///     #[derive(Debug)]
///     struct UserRecord {
///         first_name: String,
///         last_name: String,
///         age: f64
///     }
/// );
/// # }
/// ```
#[macro_export]
macro_rules! dbase_record {
    (
        $(#[derive($($derives:meta),*)])?
        struct $name:ident {
            $( $field_name:ident: $field_type:ty),+
            $(,)?
        }
    ) => {

        $(#[derive($($derives),*)])?
        struct $name {
            $($field_name: $field_type),+
        }

        impl dbase::ReadableRecord for $name {
            fn read_using<T>(field_iterator: &mut dbase::FieldIterator<T>) -> Result<Self, dbase::FieldIOError>
                where T: std::io::Read + std::io::Seek
                {
                    Ok(Self {
                        $(
                            $field_name: field_iterator
                                .read_next_field_as::<$field_type>()?
                                .value
                        ),+
                    })
            }
        }

       impl dbase::WritableRecord for $name {
           fn write_using<'a, W: std::io::Write>(&self, field_writer: &mut dbase::FieldWriter<'a, W>) -> Result<(), dbase::FieldIOError> {
                $(
                    field_writer.write_next_field_value(&self.$field_name)?;
                )+
                Ok(())
           }
        }
    };
}
