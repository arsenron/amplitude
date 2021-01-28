use thiserror::Error;

pub use amplitude::Amp;
pub use entities::Event;
pub(crate) use prelude::*;

pub mod amplitude;
pub mod entities;
pub(crate) mod prelude;
pub mod response;

type SerdeMap = serde_json::Map<String, serde_json::Value>;

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
