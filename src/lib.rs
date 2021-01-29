pub mod amp;
pub mod entities;
pub(crate) mod prelude;
pub mod response;

pub use amp::Amp;
pub use entities::Event;
use prelude::*;
use thiserror::Error;

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
