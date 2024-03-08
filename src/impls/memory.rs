use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::vec;
use crate::{Error, VFile, VFileSystem, VMetadata, VPath};

pub struct MemoryFileSystem<M: VMetadata> {
    entries: RwLock<HashMap<VPath, VFile<M>>>,
    root: VPath
}

#[allow(dead_code)]
impl<M: VMetadata> MemoryFileSystem<M> {
    pub fn new_empty(root: VPath) -> Self { Self::new(root, HashMap::new()) }

    pub fn new(root: VPath, entries: HashMap<VPath, VFile<M>>) -> Self{
        Self { entries: RwLock::new(entries), root, }
    }
}

impl<M: VMetadata> VFileSystem<M> for MemoryFileSystem<M> {
    type VPathIterator = vec::IntoIter<VPath>;

    fn fs_root(&self) -> VPath { self.root.clone() }

    fn fs_iter(&self) -> crate::Result<Self::VPathIterator> {
        // Premature collect for thread safety
        let keys: Vec<VPath> = self.entries.read().unwrap().keys().cloned().collect();
        Ok(keys.into_iter())
    }

    fn fs_insert(&mut self, path: &VPath, new_file: VFile<M>) -> crate::Result<Option<VFile<M>>> {
        let mut write_handle = self.entries.write().unwrap();

        Ok(write_handle.insert(path.clone(), new_file))
    }

    fn fs_move(&mut self, path: &VPath, new_path: VPath) -> crate::Result<Option<VFile<M>>> {
        let mut write_handle = self.entries.write().unwrap();
        let file = write_handle.remove(path).ok_or(Error::EntryNotFound)?;

        Ok(write_handle.insert(new_path, file))
    }

    fn fs_remove(&mut self, path: &VPath) -> crate::Result<VFile<M>> {
        let mut write_handle = self.entries.write().unwrap();

        write_handle.remove(path).ok_or(Error::EntryNotFound)
    }

    fn fs_copy(&mut self, path: &VPath, copy_to: VPath) -> crate::Result<Option<VFile<M>>> {
        let handle =  self.entries.read().unwrap();
        let file = handle.get(path).ok_or(Error::EntryNotFound)?.clone();
        let mut handle = self.entries.write().unwrap();
        let replaced = handle.insert(copy_to, file.clone());
        Ok(replaced)
    }

    fn fs_contents(&self, path: &VPath) -> crate::Result<Arc<[u8]>> {
        let read_handle = self.entries.read().unwrap();
        let file = read_handle.get(path).ok_or(Error::EntryNotFound)?;

        Ok(file.contents())
    }

    fn fs_meta(&self, path: &VPath) -> crate::Result<M> {
        let read_handle = self.entries.read().unwrap();
        let file = read_handle.get(path).ok_or(Error::EntryNotFound)?;

        Ok(file.metadata())
    }
}