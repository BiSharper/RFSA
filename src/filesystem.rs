use std::sync::Arc;
use crate::{PathLike, ReadableVFile, ReadableVMetadata, VDirectoryIterator, VFile, VFileContainer, VMetadata, VPath, WritableVFile, WritableVMetadata};

pub trait VFileSystem<M: VMetadata>: Sized + Send + Sync {
    type VPathIterator: Iterator<Item=VPath> + Send;

    fn fs_root(&self) -> VPath { VPath::exact("") }

    fn fs_iter(&self) -> crate::Result<Self::VPathIterator>;

    fn fs_insert(&mut self, path: &VPath, new_file: VFile<M>) -> crate::Result<Option<VFile<M>>>;

    fn fs_move(&mut self, path: &VPath, new_path: VPath) -> crate::Result<Option<VFile<M>>>;

    fn fs_remove(&mut self, path: &VPath) -> crate::Result<VFile<M>>;

    fn fs_copy(&mut self, path: &VPath, copy_to: VPath) -> crate::Result<Option<VFile<M>>>;

    fn fs_contents(&self, path: &VPath) -> crate::Result<Arc<[u8]>>;

    fn fs_meta(&self, path: &VPath) -> crate::Result<M>;

    fn dir_iter(&self, path: &VPath, recursive: bool) -> crate::Result<VDirectoryIterator<M, Self>> {
        Ok(VDirectoryIterator::create(self.fs_iter()?, path.clone(), recursive))
    }
}

impl<M: VMetadata, F: VFileSystem<M>> VFileContainer<M, Self> for F {
    fn dir_root(&self) -> VPath { self.fs_root() }

    fn file_read(&self, path: &VPath) -> crate::Result<ReadableVFile<M>> {
        let contents = self.fs_contents(path)?;
        Ok(ReadableVFile::with(self.meta_read(path)?, contents, 0))
    }

    fn file_write(&mut self, path: &VPath) -> crate::Result<WritableVFile<M, Self>> {
        let contents = self.fs_contents(path)?.to_vec();
        Ok(WritableVFile::with(self.meta_write(path)?, contents))
    }

    fn file_create(&mut self, path: &VPath) -> crate::Result<WritableVFile<M, Self>> {
        self.file_replace(path, VFile::create(M::default(), Arc::new([0])))?;
        self.file_write(&path)
    }

    fn file_replace(&mut self, path: &VPath, new_file: VFile<M>) -> crate::Result<Option<VFile<M>>> {
        self.fs_insert(path, new_file)
    }

    fn file_move(&mut self, path: &VPath, new_path: &VPath) -> crate::Result<Option<VFile<M>>> {
        let file = self.fs_remove(path)?;
        self.fs_insert(new_path, file)
    }

    fn file_copy(&mut self, path: &VPath, copy_to: &VPath) -> crate::Result<Option<VFile<M>>> {
        let file = VFile::create(self.fs_meta(path)?, self.fs_contents(path)?);
        self.fs_insert(copy_to, file)
    }

    fn meta_read(&self, path: &VPath) -> crate::Result<ReadableVMetadata<M>> {
        Ok(ReadableVMetadata::with(self.fs_meta(path)?))
    }

    fn meta_write(&mut self, path: &VPath) -> crate::Result<WritableVMetadata<M, Self>> {
        Ok(WritableVMetadata::with(self, path.clone(), self.fs_meta(path)?))
    }

    fn dir_iter(&self, path: &VPath, recursive: bool) -> crate::Result<VDirectoryIterator<M, Self>> {
        self.dir_iter(path, recursive)
    }
}