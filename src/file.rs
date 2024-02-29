use std::cmp;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use crate::VMetadata;

#[derive(Clone, Eq, PartialEq)]
pub struct VFile<M: VMetadata> {
    metadata: M,
    contents: Arc<[u8]>
}

impl<M: VMetadata> VFile<M> {
    pub fn create(metadata: M, contents: Vec<u8>) -> Self{
        Self { metadata, contents: Arc::from(contents.into_boxed_slice()) }
    }


    pub fn create_empty(metadata: M) -> Self {
        Self::create(metadata, vec![])
    }
}

pub struct ReadableVMetadata<'a, M: VMetadata> {
    file: &'a VFile<M>
}

impl<'a, M: VMetadata> ReadableVMetadata<'a, M> {
    pub fn new(file: &'a VFile<M>) -> Self { Self { file } }

    pub fn buf_len(&self) -> usize { self.file.contents.len() }
}

impl<'a, M: VMetadata> Into<ReadableVFile<'a, M>> for ReadableVMetadata<'a, M> {
    fn into(self) -> ReadableVFile<'a, M> { ReadableVFile::new(self.file, 0) }
}

impl<'a, M: VMetadata> Deref for ReadableVMetadata<'a, M> {
    type Target = M;

    fn deref(&self) -> &Self::Target { &self.file.metadata }
}

pub struct ReadableVFile<'a, M: VMetadata> {
    metadata: ReadableVMetadata<'a, M>,
    position: usize
}

impl<'a, M: VMetadata> ReadableVFile<'a, M>  {
    pub fn new(file: &'a VFile<M>, position: usize) -> Self { Self::from_meta(ReadableVMetadata::new(file), position) }

    pub fn from_meta(metadata: ReadableVMetadata<'a, M>, position: usize) -> Self {
        Self { metadata, position, }
    }
}

impl<'a, M: VMetadata> Deref for ReadableVFile<'a, M> {
    type Target = ReadableVMetadata<'a, M>;

    fn deref(&self) -> &Self::Target { &self.metadata }
}

impl<'a, M: VMetadata> Read for ReadableVFile<'a, M> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let amt = cmp::min(buf.len(), self.buf_len());

        if amt == 1 {
            buf[0] = self.file.contents[self.position];
        } else {
            buf[..amt].copy_from_slice(
                &self.file.contents[self.position..self.position + amt],
            );
        }
        self.position += amt;
        Ok(amt)
    }
}

impl<'a, M: VMetadata> Seek for ReadableVFile<'a, M> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(offset) => self.position = offset as usize,
            SeekFrom::Current(offset) => self.position = self.position + offset as usize,
            SeekFrom::End(offset) => self.position = self.file.contents.len() + offset as usize,
        }
        Ok(self.position as u64)
    }

    fn stream_position(&mut self) -> std::io::Result<u64> { Ok(self.position as u64) }
}


pub struct WritableVMetadata<'a, M: VMetadata> { file: &'a mut VFile<M>, }

impl<'a, M: VMetadata> WritableVMetadata<'a, M> {
    pub fn new(file: &'a mut VFile<M>) -> Self { Self { file } }
}

impl<'a, M: VMetadata> Into<WritableVFile<'a, M>> for WritableVMetadata<'a, M> {
    fn into(self) -> WritableVFile<'a, M> { WritableVFile::new(self.file) }
}

impl<'a, M: VMetadata> Deref for WritableVMetadata<'a, M> {
    type Target = M;

    fn deref(&self) -> &Self::Target { &self.file.metadata }
}

impl<'a, M: VMetadata> DerefMut for WritableVMetadata<'a, M> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.file.metadata }
}

pub struct WritableVFile<'a, M: VMetadata> {
    metadata: WritableVMetadata<'a, M>,
    cursor:   Cursor<Vec<u8>>
}

impl<'a, M: VMetadata> WritableVFile<'a, M>  {
    pub fn new(file: &'a mut VFile<M>) -> Self { Self::with_content(file, file.contents.to_vec()) }


    pub fn with_content(file: &'a mut VFile<M>, contents: Vec<u8>) -> Self {
        Self { metadata: WritableVMetadata { file }, cursor: Cursor::new(contents), }
    }
}

impl<'a, M: VMetadata> Deref for WritableVFile<'a, M> {
    type Target = WritableVMetadata<'a, M>;

    fn deref(&self) -> &Self::Target { &self.metadata }
}

impl<'a, M: VMetadata> DerefMut for WritableVFile<'a, M> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.metadata }
}

impl<'a, M: VMetadata> AsRef<Cursor<Vec<u8>>> for WritableVFile<'a, M> {
    fn as_ref(&self) -> &Cursor<Vec<u8>> { &self.cursor }
}

impl<'a, M: VMetadata> AsMut<Cursor<Vec<u8>>> for WritableVFile<'a, M> {
    fn as_mut(&mut self) -> &mut Cursor<Vec<u8>> { &mut self.cursor }
}

impl<'a, M: VMetadata> Write for WritableVFile<'a, M> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.cursor.write(buf) }

    fn flush(&mut self) -> std::io::Result<()> { self.cursor.flush() }
}

impl<'a, M: VMetadata> Seek for WritableVFile<'a, M> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> { self.cursor.seek(pos) }

    fn stream_position(&mut self) -> std::io::Result<u64> { self.cursor.stream_position() }
}

impl<'a, M: VMetadata> Read for WritableVFile<'a, M> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> { self.cursor.read(buf) }
}

impl<'a, M: VMetadata> Drop for WritableVFile<'a, M> {
    fn drop(&mut self) {
        self.file.contents = Arc::from(self.cursor.get_ref().clone().into_boxed_slice())
    }
}