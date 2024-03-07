use std::marker::PhantomData;
use crate::{GFS_SEPARATOR, ReadableVFile, ReadableVMetadata, VFile, VFileSystem, VMetadata, WritableVFile, WritableVMetadata};
use crate::error::VFSResult;
use crate::path::{PathLike, VPath};

pub struct VDirectory<'a, M: VMetadata, F: VFileSystem<M>> {
    filesystem: &'a mut F,
    path:       VPath,
    marker:     PhantomData<M>
}

pub struct VDirectoryIterator<M: VMetadata, F: VFileSystem<M>> {
    inner:      F::VPathIterator,
    recursive:  bool,
    prefix:     String,
}

impl<M: VMetadata, F: VFileSystem<M>> VDirectoryIterator<M, F> {
    pub fn create(inner: F::VPathIterator, prefix: String, recursive: bool) -> Self {
        Self { inner, recursive, prefix }
    }
}

impl<M: VMetadata, F: VFileSystem<M>> Iterator for VDirectoryIterator<M, F> {
    type Item = VPath;

    fn next(&mut self) -> Option<Self::Item> { match self.inner.next() {
        Some(candidate) if candidate.starts_with(&self.prefix) && (
            !self.recursive ||
            !candidate[self.prefix.len()..].contains(GFS_SEPARATOR)
        ) => Some(candidate),
        _ => None
    } }
}

pub trait VFileContainer<M: VMetadata, F: VFileSystem<M>> : Sync + Send {

    fn root(&self) -> &VPath;

    fn file_read(&self, path: &VPath) -> VFSResult<ReadableVFile<M>>;

    fn file_write(&mut self, path: &VPath) -> VFSResult<WritableVFile<M, F>>;

    fn file_create(&mut self, path: &VPath) -> VFSResult<WritableVFile<M, F>>;

    fn meta_read(&self, path: &VPath) -> VFSResult<ReadableVMetadata<M>>;

    fn meta_write(&mut self, path: &VPath) -> VFSResult<WritableVMetadata<M, F>>;

    fn dir_iter(&self, path: &VPath, recursive: bool) -> VFSResult<VDirectoryIterator<M, F>>;
}

impl<'a, M: VMetadata, F: VFileSystem<M>> VDirectory<'a, M, F> {
    pub fn create(filesystem: &'a mut F, path: VPath) -> VDirectory<'a, M, F> {
        Self { filesystem, path, marker: PhantomData, }
    }
}
impl<'a, M: VMetadata, F: VFileSystem<M>> VDirectory<'a, M, F> {
    pub fn file_remove(&mut self, path: &VPath) -> VFSResult<Option<(VPath, VFile<M>)>> {
        self.filesystem.file_remove(&self.path.join(path))
    }

    pub fn file_exists(&self, path: &VPath) -> VFSResult<bool> {
        self.filesystem.file_exists(&self.path.join(path))
    }

    pub fn dir_exists(&self, path: &VPath) -> VFSResult<bool> {
        self.filesystem.dir_exists(&self.path.join(path))
    }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> VFileContainer<M, F> for VDirectory<'a, M, F> {

    fn root(&self) -> &VPath { &self.path }

    fn file_read(&self, path: &VPath) -> VFSResult<ReadableVFile<M>> {
        self.filesystem.file_read(&self.root().join(path))
    }

    fn file_write(&mut self, path: &VPath) -> VFSResult<WritableVFile<M, F>> {
        self.filesystem.file_write(&self.root().join(path))
    }

    fn file_create(&mut self, path: &VPath) -> VFSResult<WritableVFile<M, F>> {
        self.filesystem.file_create(&self.root().join(path))
    }

    fn meta_read(&self, path: &VPath) -> VFSResult<ReadableVMetadata<M>> {
        self.filesystem.meta_read(&self.root().join(path))
    }

    fn meta_write(&mut self, path: &VPath) -> VFSResult<WritableVMetadata<M, F>> {
        self.filesystem.meta_write(&self.root().join(path))
    }

    fn dir_iter(&self, path: &VPath, recursive: bool) -> VFSResult<VDirectoryIterator<M, F>> {
        self.filesystem.path_iter(self.root().join(path).as_directory_string(), recursive)
    }

}