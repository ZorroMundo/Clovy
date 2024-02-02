use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::sprite::Sprite};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct ChunkSPRT {
    pub sprites: GMPointerList<Sprite>,
}

impl Serialize for ChunkSPRT {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.sprites.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.sprites.serialize(writer, Some(Box::new(|writer: &mut Writer<W>, _index, _count| {
            writer.pad(4).expect("Failed to pad writer");
        })), None);
    }
}
