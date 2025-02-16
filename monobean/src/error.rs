use thiserror::Error;
use common::errors::{MegaError, ProtocolError};

pub type MonoBeanResult<T> = Result<T, MonoBeanError>;

#[derive(Error, Debug)]
pub enum MonoBeanError {
    #[error("Mega Core Error: {0}")]
    MegaCoreError(String),
    
    #[error("Mega Protocol Error: {0}")]
    MegaProtocolError(#[from] ProtocolError),
    
    #[error("Mega Server Error: {0}")]
    MegaServerError(#[from] std::io::Error),
}