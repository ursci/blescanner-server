//! Defining custom errors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RmError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
