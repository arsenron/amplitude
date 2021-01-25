pub(crate) mod prelude;
pub mod amplitude;
pub mod entities;
pub mod response;

pub(crate) use prelude::*;
pub use amplitude::Amp;
pub use entities::Event;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AmplitudeError {
    #[error("initialization error")]
    InitializationError(String),

    #[error("A network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),

    #[error("unknown error")]
    UnknownError,
}


