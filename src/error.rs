use std::{io, result};
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error")]
    IoError(#[from] io::Error),
    #[error("The specified entry was not found")]
    EntryNotFound,
}
