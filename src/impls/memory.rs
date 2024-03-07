use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::{VFile, VFileSystem, VFSError, VMetadata, VPath};
use crate::error::{VFSResult};

pub struct MemoryFileSystem<M: VMetadata> {
    inner: RwLock<HashMap<VPath, VFile<M>>>,
    root: VPath
}

impl<M: VMetadata> VFileSystem<M> for MemoryFileSystem<M> {
    type VPathIterator = std::vec::IntoIter<VPath>;

    fn fs_root(&self) -> &VPath { &self.root }

    fn root_iter(&self) -> VFSResult<Self::VPathIterator> {
        // Premature collect for thread safety
        let keys: Vec<VPath> = self.inner.read().unwrap().keys().cloned().collect();
        Ok(keys.into_iter())
    }

    fn file_remove(&mut self, path: &VPath) -> VFSResult<Option<(VPath, VFile<M>)>> {
        let mut write_lock = self.inner.write().unwrap();
        Ok(write_lock.remove_entry(path))
    }

    fn file_exists(&self, path: &VPath) -> VFSResult<bool> {
        let hashmap = self.inner.read().unwrap();
        Ok(hashmap.contains_key(path))
    }

    fn file_insert(&mut self, path: &VPath, file: VFile<M>) -> VFSResult<Option<VFile<M>>> {
        Ok(self.inner.write().unwrap().insert(path.clone(), file))
    }

    fn file_contents(&self, path: &VPath) -> VFSResult<Arc<[u8]>> {
        let handle = self.inner.read().unwrap();
        Ok(handle.get(path).ok_or(VFSError::EntryNotFound)?.contents())
    }

    fn file_meta(&self, path: &VPath) -> VFSResult<M> {
        let handle = self.inner.read().unwrap();
        Ok(handle.get(path).ok_or(VFSError::EntryNotFound)?.metadata())
    }
}