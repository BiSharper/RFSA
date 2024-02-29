extern crate core;
pub extern crate rfsa_macros as macros;

pub mod error;
mod path;
pub use path::*;
mod metadata; pub use metadata::*;
mod file; pub use file::*;
mod directory; pub use directory::*;
use crate::error::VFSResult;


pub const FILESYSTEM_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub trait VFileSystem<M: VMetadata>: Sized + Send + Sync + 'static {
    type VPathIterator<'fs>: Iterator<Item=&'fs VPath> + Send + 'fs;

    #[allow(clippy::needless_lifetimes)]
    fn paths<'fs>(&'fs self) -> VFSResult<Self::VPathIterator<'fs>>;

    fn path_iter(&self,
                 path_prefix: String,
                 recursive: bool
    ) -> VFSResult<Box<dyn Iterator<Item = &VPath> + Send + '_>> {
        let prefix_len = path_prefix.len();

        Ok(Box::new(self.paths()?.filter(move |candidate: &&VPath| {
            candidate.starts_with(path_prefix.as_str()) &&
                (!recursive || !candidate[prefix_len..].contains(GFS_SEPARATOR))
        })))
    }

    fn file_remove(&mut self, path: &VPath) -> VFSResult<VFile<M>>;

    fn file_exists(&self, directory: &VPath) -> VFSResult<bool>;

    fn file_insert(&mut self, path: &VPath, file: VFile<M>) -> VFSResult<Option<VFile<M>>>;

    fn file_mut(&mut self, path: &VPath) -> VFSResult<&mut VFile<M>>;

    fn file_get(&self, path: &VPath) -> VFSResult<&VFile<M>>;

    fn fs_root(&self) -> VPath { VPath::create(GFS_ROOT) }

    fn dir_exists(&self, directory: &VPath) -> VFSResult<bool> {
        let directory_prefix = directory.as_directory_string();
        Ok(self.paths()?.find(|p| {
            p.starts_with(&directory_prefix)
        }) != None)
    }

    fn dir_remove(&mut self, directory: &VPath) -> VFSResult<Box<[(VPath, VFile<M>)]>>;

}

impl<M: VMetadata, T: VFileSystem<M>> VFileContainer<M, Self> for T {
    fn root(&self) -> VPath { self.fs_root() }

    fn file_read(&self, path: &VPath) -> VFSResult<ReadableVFile<M>> {
        Ok(ReadableVFile::new(self.file_get(path)?, 0))
    }

    fn file_write(&mut self, path: &VPath) -> VFSResult<WritableVFile<M>> {
        Ok(WritableVFile::new(self.file_mut(path)?))
    }

    fn file_create(&mut self, path: &VPath) -> VFSResult<WritableVFile<M>> {
        self.file_insert(path, VFile::create_empty(M::default()))?;
        Ok(WritableVFile::new(self.file_mut(path)?))
    }

    fn meta_read(&self, path: &VPath) -> VFSResult<ReadableVMetadata<M>> {
        Ok(ReadableVMetadata::new(self.file_get(path)?))
    }

    fn meta_write(&mut self, path: &VPath) -> VFSResult<WritableVMetadata<M>> {
        Ok(WritableVMetadata::new(self.file_mut(path)?))
    }

    fn dir_iter(&self, path: &VPath, recursive: bool) -> VFSResult<Box<dyn Iterator<Item=&VPath> + Send + '_>> {
        self.path_iter(path.as_directory_string(), recursive)
    }
}
