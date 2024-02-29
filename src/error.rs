use std::io;
use thiserror::Error;

pub type VFSResult<T> = Result<T, VFSError>;

#[derive(Error, Debug)]
pub enum VFSError {
    #[error("IO Error")]
    IoError(#[from] io::Error),
    #[error("The specified entry was not found")]
    EntryNotFound,
    #[error("Filesystem Error Occurred: {0}")]
    Other(String),
}
