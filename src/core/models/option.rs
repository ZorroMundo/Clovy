use std::{fmt::Write, io::{Read, Seek}};
use bstr::BString;
use byteorder::WriteBytesExt;
use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};

#[derive(Default, Clone)]
pub struct Constant {
    pub name: BString,
    pub value: BString,
}

impl Serialize for Constant {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.value = reader.read_pointer_string().expect("Failed to read value");

        chunk
    }
    
    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_pointer_string(&chunk.value).expect("Failed to write value");
    }
}
