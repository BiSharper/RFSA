extern crate core;
pub extern crate rfsa_macros as macros;


use std::sync::Arc;
mod error; pub use error::*;
mod path; pub use path::*;
mod metadata; pub use metadata::*;
mod file; pub use file::*;
mod directory; pub use directory::*;
pub mod impls;

pub const FILESYSTEM_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub trait VFileSystem<M: VMetadata>: Sized + Send + Sync + 'static {
    type VPathIterator: Iterator<Item=VPath> + Send;

    fn fs_root(&self) -> &VPath;

    fn root_iter(&self) -> VFSResult<Self::VPathIterator>;

    fn path_iter(&self, path_prefix: String, recursive: bool) -> VFSResult<VDirectoryIterator<M, Self>> {
        Ok(VDirectoryIterator::create(self.root_iter()?, path_prefix, recursive))
    }

    fn file_remove(&mut self, path: &VPath) -> VFSResult<Option<(VPath, VFile<M>)>>;

    fn file_exists(&self, path: &VPath) -> VFSResult<bool>;

    fn file_insert(&mut self, path: &VPath, file: VFile<M>) -> VFSResult<Option<VFile<M>>>;

    fn file_contents(&self, path: &VPath) -> VFSResult<Arc<[u8]>>;

    fn file_meta(&self, path: &VPath) -> VFSResult<M>;

    fn dir_exists(&self, path: &VPath) -> VFSResult<bool> {
        let directory_prefix = path.as_directory_string();
        Ok(self.root_iter()?.find(|p| {
            p.starts_with(&directory_prefix)
        }) != None)
    }
}

impl<M: VMetadata, F: VFileSystem<M>> VFileContainer<M, Self> for F {
    fn root(&self) -> &VPath { &self.fs_root() }

    fn file_read(&self, path: &VPath) -> VFSResult<ReadableVFile<M>> {
        let contents = self.file_contents(path)?;
        Ok(ReadableVFile::new(self.meta_read(path)?, contents, 0))
    }

    fn file_write(&mut self, path: &VPath) -> VFSResult<WritableVFile<M, Self>> {
        let contents = self.file_contents(path)?.to_vec();
        Ok(WritableVFile::new(self.meta_write(path)?, contents))
    }

    fn file_create(&mut self, path: &VPath) -> VFSResult<WritableVFile<M, Self>> {
        self.file_insert(path, VFile::create(M::default(), vec![]))?;
        self.file_write(&path)
    }

    fn meta_read(&self, path: &VPath) -> VFSResult<ReadableVMetadata<M>> {
        Ok(ReadableVMetadata::new(self.file_meta(path)?))
    }

    fn meta_write(&mut self, path: &VPath) -> VFSResult<WritableVMetadata<M, Self>> {
        Ok(WritableVMetadata::new(self, path.clone(), self.file_meta(path)?))
    }

    fn dir_iter(&self, path: &VPath, recursive: bool) -> VFSResult<VDirectoryIterator<M, Self>> {
        self.path_iter(path.as_directory_string(), recursive)
    }
}