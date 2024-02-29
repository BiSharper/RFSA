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

pub trait VFileSystem<M: VMetadata>: VFileContainer<M, Self> + Sized + Send + Sync + 'static {
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

    fn dir_exists(&self, directory: &VPath) -> VFSResult<bool> {
        let directory_prefix = directory.as_directory_string();
        Ok(self.paths()?.find(|p| {
            p.starts_with(&directory_prefix)
        }) != None)
    }

    fn dir_remove(&mut self, directory: &VPath) -> VFSResult<Box<[(VPath, VFile<M>)]>>;

}
