pub trait VMetadata : Sized + Clone + Copy + Eq + PartialEq + Default + Send + Sync {}

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct NoMetaData;

impl VMetadata for NoMetaData {
    
}