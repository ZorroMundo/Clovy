use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::script::Script};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct ChunkSCPT {
    pub scripts: GMPointerList<Script>,
}

impl Serialize for ChunkSCPT {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.scripts.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.scripts.serialize(writer, None, None)?;

        Ok(())
    }
}
