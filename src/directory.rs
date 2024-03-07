use std::marker::PhantomData;
use crate::{PathLike, ReadableVFile, ReadableVMetadata, VFile, VFileSystem, VMetadata, VPath, WritableVFile, WritableVMetadata};

pub struct VDirectoryIterator<M: VMetadata, F: VFileSystem<M>> {
    inner:      F::VPathIterator,
    recursive:  bool,
    path:       String,
}

pub struct VDirectory<'a, M: VMetadata, F: VFileSystem<M>> {
    filesystem: &'a mut F,
    path:       VPath,
    marker:     PhantomData<M>
}

pub trait VFileContainer<M: VMetadata, F: VFileSystem<M>> : Sync + Send {

    fn dir_root(&self) -> VPath;

    fn file_read(&self, path: &VPath) -> crate::Result<ReadableVFile<M>>;

    fn file_write(&mut self, path: &VPath) -> crate::Result<WritableVFile<M, F>>;

    fn file_create(&mut self, path: &VPath) -> crate::Result<WritableVFile<M, F>>;

    fn file_replace(&mut self, path: &VPath, new_file: VFile<M>) -> crate::Result<Option<VFile<M>>>;

    fn file_move(&mut self, path: &VPath, new_path: &VPath) -> crate::Result<Option<VFile<M>>>;

    fn file_copy(&mut self, path: &VPath, copy_to: &VPath) -> crate::Result<Option<VFile<M>>>;

    fn meta_read(&self, path: &VPath) -> crate::Result<ReadableVMetadata<M>>;

    fn meta_write(&mut self, path: &VPath) -> crate::Result<WritableVMetadata<M, F>>;

    fn dir_iter(&self, path: &VPath, recursive: bool) -> crate::Result<VDirectoryIterator<M, F>>;
}

impl<M: VMetadata, F: VFileSystem<M>> VDirectoryIterator<M, F> {
    pub fn create(inner: F::VPathIterator, path: VPath, recursive: bool) -> Self {
        Self { inner, recursive, path: path.as_directory_string(), }
    }
}

impl<M: VMetadata, F: VFileSystem<M>> Iterator for VDirectoryIterator<M, F> {
    type Item = VPath;

    fn next(&mut self) -> Option<Self::Item> { match self.inner.next() {
        Some(candidate) if candidate.starts_with(&self.path) && (
            !self.recursive || !candidate[self.path.len()..].contains(crate::SEPARATOR)
        ) => Some(candidate),
        _ => None
    } }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> VDirectory<'a, M, F> {
    pub fn create(filesystem: &'a mut F, path: VPath) -> VDirectory<'a, M, F> {
        Self { filesystem, path, marker: PhantomData, }
    }
}

impl<'a, M: VMetadata, F: VFileSystem<M>> VFileContainer<M, F> for VDirectory<'a, M, F> {
    fn dir_root(&self) -> VPath { self.path.clone() }

    fn file_read(&self, path: &VPath) -> crate::Result<ReadableVFile<M>> {
        self.filesystem.file_read(&self.dir_root().join_into(path))
    }

    fn file_write(&mut self, path: &VPath) -> crate::Result<WritableVFile<M, F>> {
        self.filesystem.file_write(&self.dir_root().join_into(path))
    }

    fn file_create(&mut self, path: &VPath) -> crate::Result<WritableVFile<M, F>> {
        self.filesystem.file_create(&self.dir_root().join_into(path))
    }

    fn file_replace(&mut self, path: &VPath, new_file: VFile<M>) -> crate::Result<Option<VFile<M>>> {
        self.filesystem.file_replace(&self.dir_root().join_into(path), new_file)
    }

    fn file_move(&mut self, path: &VPath, move_to: &VPath) -> crate::Result<Option<VFile<M>>> {
        self.filesystem.file_move(&self.dir_root().join_into(path), move_to)
    }

    fn file_copy(&mut self, path: &VPath, copy_to: &VPath) -> crate::Result<Option<VFile<M>>> {
        self.filesystem.file_copy(&self.dir_root().join_into(path), copy_to)
    }

    fn meta_read(&self, path: &VPath) -> crate::Result<ReadableVMetadata<M>> {
        self.filesystem.meta_read(&self.dir_root().join_into(path))
    }

    fn meta_write(&mut self, path: &VPath) -> crate::Result<WritableVMetadata<M, F>> {
        self.filesystem.meta_write(&self.dir_root().join_into(path))
    }

    fn dir_iter(&self, path: &VPath, recursive: bool) -> crate::Result<VDirectoryIterator<M, F>> {
        self.filesystem.dir_iter(&self.dir_root().join_into(path), recursive)
    }
}