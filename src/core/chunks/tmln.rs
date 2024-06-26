use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::timeline::Timeline};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct ChunkTMLN {
    pub timelines: GMPointerList<Timeline>,
}

impl Serialize for ChunkTMLN {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.timelines.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.timelines.serialize(writer, None, None)?;

        Ok(())
    }
}
