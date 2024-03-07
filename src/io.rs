use std::cmp;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use crate::{VFileSystem, VMetadata, VPath};

#[derive(Clone, Eq, PartialEq)]
pub struct VFile<M: VMetadata> { metadata: M, contents: Arc<[u8]> }

impl<M: VMetadata> VFile<M> {
    pub fn create(metadata: M, contents: Arc<[u8]>) -> Self{ Self { metadata, contents } }
    pub fn metadata(&self) -> M { self.metadata.clone() }
    pub fn contents(&self) -> Arc<[u8]> { self.contents.clone() }
}

pub struct ReadableVMetadata<M: VMetadata>(M);

impl<M: VMetadata> ReadableVMetadata<M> {
    pub fn with(metadata: M) -> Self { Self { 0: metadata } }
}

impl<M: VMetadata> Deref for ReadableVMetadata<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target { &self.0 }
}

pub struct ReadableVFile<M: VMetadata> {
    readable_metadata: ReadableVMetadata<M>,
    contents: Arc<[u8]>,
    position: usize
}

impl<'a, M: VMetadata> ReadableVFile<M>  {
    pub fn with(readable_metadata: ReadableVMetadata<M>, contents: Arc<[u8]>, position: usize) -> Self {
        Self { readable_metadata, contents, position }
    }
}

impl<M: VMetadata> Deref for ReadableVFile<M> {
    type Target = ReadableVMetadata<M>;

    fn deref(&self) -> &Self::Target { &self.readable_metadata }
}

impl<M: VMetadata> Read for ReadableVFile<M> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let amt = cmp::min(buf.len(), self.contents.len());

        if amt == 1 {
            buf[0] = self.contents[self.position];
        } else {
            buf[..amt].copy_from_slice(
                &self.contents[self.position..self.position + amt],
            );
        }
        self.position += amt;
        Ok(amt)
    }
}

impl<M: VMetadata> Seek for ReadableVFile<M> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(offset) => self.position = offset as usize,
            SeekFrom::Current(offset) => self.position = self.position + offset as usize,
            SeekFrom::End(offset) => self.position = self.contents.len() + offset as usize,
        }
        Ok(self.position as u64)
    }

    fn stream_position(&mut self) -> std::io::Result<u64> { Ok(self.position as u64) }
}

pub struct WritableVMetadata<'a, M: VMetadata, F: VFileSystem<M>> {
    filesystem: &'a mut F,
    path: VPath,
    metadata: M
}

impl<'a, M: VMetadata, F: VFileSystem<M>> WritableVMetadata<'a, M, F> {
    pub fn with(filesystem: &'a mut F, path: VPath, metadata: M) -> Self { Self { filesystem, path, metadata } }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> Deref for WritableVMetadata<'a, M, F> {
    type Target = M;

    fn deref(&self) -> &Self::Target { &self.metadata }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> DerefMut for WritableVMetadata<'a, M, F> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.metadata }
}

pub struct WritableVFile<'a, M: VMetadata, F: VFileSystem<M>> {
    writable_metadata: WritableVMetadata<'a, M, F>,
    cursor:            Cursor<Vec<u8>>
}

impl<'a, M: VMetadata, F: VFileSystem<M>> WritableVFile<'a, M, F>  {
    pub fn with(writable_metadata: WritableVMetadata<'a, M, F>, contents: Vec<u8>) -> Self {
        Self { writable_metadata, cursor: Cursor::new(contents), }
    }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> Deref for WritableVFile<'a, M, F> {
    type Target = WritableVMetadata<'a, M, F>;

    fn deref(&self) -> &Self::Target { &self.writable_metadata }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> DerefMut for WritableVFile<'a, M, F> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.writable_metadata }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> Write for WritableVFile<'a, M, F> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.cursor.write(buf) }

    fn flush(&mut self) -> std::io::Result<()> { self.cursor.flush() }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> Seek for WritableVFile<'a, M, F> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> { self.cursor.seek(pos) }

    fn stream_position(&mut self) -> std::io::Result<u64> { self.cursor.stream_position() }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> Read for WritableVFile<'a, M, F> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> { self.cursor.read(buf) }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> Drop for WritableVFile<'a, M, F> {
    fn drop(&mut self) {
        let path = self.path.clone();
        let contents = self.cursor.get_ref().clone().into_boxed_slice();
        let file = VFile::create(self.metadata.clone(), Arc::from(contents));
        self.filesystem.fs_insert(&path, file).unwrap();
    }
}