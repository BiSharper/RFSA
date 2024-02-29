use std::marker::PhantomData;
use crate::{ReadableVFile, ReadableVMetadata, VFile, VFileSystem, VMetadata, WritableVFile, WritableVMetadata};
use crate::error::VFSResult;
use crate::path::{PathLike, VPath};

pub type VPathIterator = Box<dyn Iterator<Item=VPath> + Send>;

pub struct VDirectory<'a, M: VMetadata, F: VFileSystem<M>> {
    filesystem: &'a mut F,
    path:       VPath,
    marker:     PhantomData<M>
}

pub trait VFileContainer<M: VMetadata, F: VFileSystem<M>> : Sync + Send {
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

    fn file_remove(&mut self, path: &VPath) -> VFSResult<VFile<M>>;

    fn file_exists(&self, path: &VPath) -> VFSResult<bool>;

    fn file_insert(&mut self, path: &VPath, file: VFile<M>) -> VFSResult<Option<VFile<M>>>;

    fn file_mut(&mut self, path: &VPath) -> VFSResult<&mut VFile<M>>;

    fn file_get(&self, path: &VPath) -> VFSResult<&VFile<M>>;

    fn meta_read(&self, path: &VPath) -> VFSResult<ReadableVMetadata<M>> {
        Ok(ReadableVMetadata::new(self.file_get(path)?))
    }

    fn meta_write(&mut self, path: &VPath) -> VFSResult<WritableVMetadata<M>> {
        Ok(WritableVMetadata::new(self.file_mut(path)?))
    }
    fn dir_exists(&self, path: &VPath) -> VFSResult<bool>;
}

impl<'a, M: VMetadata, F: VFileSystem<M>> VDirectory<'a, M, F> {
    pub fn create(filesystem: &'a mut F, path: VPath) -> VDirectory<'a, M, F> {
        Self {
            filesystem,
            path,
            marker: PhantomData,
        }
    }

    pub fn dir_iter(&self, path: &VPath, recursive: bool) -> VFSResult<VPathIterator> {
        self.filesystem.path_iter(self.path.join(path).as_directory_string(), recursive)
    }

}


impl<'a, M: VMetadata, F: VFileSystem<M>> VFileContainer<M, F> for VDirectory<'a, M, F> {

    fn file_read(&self, path: &VPath) -> VFSResult<ReadableVFile<M>> {
        self.filesystem.file_read(&self.path.join(path))
    }

    fn file_write(&mut self, path: &VPath) -> VFSResult<WritableVFile<M>> {
        self.filesystem.file_write(&self.path.join(path))
    }

    fn file_create(&mut self, path: &VPath) -> VFSResult<WritableVFile<M>> {
        self.filesystem.file_create(&self.path.join(path))
    }

    fn file_remove(&mut self, path: &VPath) -> VFSResult<VFile<M>> {
        self.filesystem.file_remove(&self.path.join(path))
    }

    fn file_exists(&self, path: &VPath) -> VFSResult<bool> {
        self.filesystem.file_exists(&self.path.join(path))
    }

    fn file_insert(&mut self, path: &VPath, file: VFile<M>) -> VFSResult<Option<VFile<M>>> {
        self.filesystem.file_insert(&self.path.join(path), file)
    }

    fn file_mut(&mut self, path: &VPath) -> VFSResult<&mut VFile<M>> {
        self.filesystem.file_mut(&self.path.join(path))
    }

    fn file_get(&self, path: &VPath) -> VFSResult<&VFile<M>> {
        self.filesystem.file_get(&self.path.join(path))
    }

    fn meta_read(&self, path: &VPath) -> VFSResult<ReadableVMetadata<M>> {
        self.filesystem.meta_read(&self.path.join(path))
    }

    fn meta_write(&mut self, path: &VPath) -> VFSResult<WritableVMetadata<M>> {
        self.filesystem.meta_write(&self.path.join(path))
    }

    fn dir_exists(&self, path: &VPath) -> VFSResult<bool> {
        self.filesystem.dir_exists(&self.path.join(path))
    }
}