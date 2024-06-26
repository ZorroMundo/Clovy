use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::audio_group::AudioGroup};
use byteorder::WriteBytesExt;
use std::{collections::HashMap, fmt::Write, fs::File, io::{BufReader, BufWriter, Cursor, Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct ChunkAGRP {
    pub audio_groups: GMPointerList<AudioGroup>,
    pub audio_data: HashMap<usize, Reader<Cursor<Vec<u8>>>>,
}

impl Serialize for ChunkAGRP {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.audio_groups.deserialize(reader, None, None)?;
        if let Some(path) = reader.path.as_ref() {
            for i in 1..chunk.audio_groups.len() {
                let mut filepath = path.clone();
                filepath.set_file_name(format!("audiogroup{i}.dat"));
                let mut f = BufReader::new(File::open(&filepath)?);
                let mut v = Vec::new();
                f.read_to_end(&mut v)?;
                drop(f);
                let mut r = Reader::new(Cursor::new(v), Some(filepath));
                r.deserialize_chunks().unwrap();
                r.deserialize().unwrap();
                chunk.audio_data.insert(i, r);
            }
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.audio_groups.serialize(writer, None, None)?;
        if let Some(path) = writer.path.as_ref() {
            for (i, data) in chunk.audio_data.iter() {
                let mut filepath = path.clone();
                filepath.set_file_name(format!("audiogroup{i}.dat"));
                let f = BufWriter::new(File::open(&filepath)?);
                let _r = Writer::from_reader(f, data, Some(filepath));
                // TODO: Finish this
            }
        }

        Ok(())
    }
}
