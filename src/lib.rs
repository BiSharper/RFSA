extern crate core;
pub extern crate rfsa_macros as macros;

pub mod error;
mod path; pub use path::*;
mod metadata; pub use metadata::*;
mod file; pub use file::*;
mod directory; pub use directory::*;
use crate::error::VFSResult;


pub const FILESYSTEM_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub trait VFileSystem<M: VMetadata>: VFileContainer<M, Self> + Sized + Send + Sync + 'static {
    fn paths(&self) -> VFSResult<VPathIterator>;

    fn path_iter(&self, path_prefix: String, recursive: bool) -> VFSResult<VPathIterator>;

    fn dir_open(&mut self, path: &VPath) ->  VFSResult<VDirectory<M, Self>> {
        Ok(VDirectory::create(self, path.clone()))
    }

}
