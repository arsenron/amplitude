#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub(crate) mod prelude;
pub mod amp;
pub mod entities;
pub mod response;
pub mod errors;

pub(crate) use prelude::*;
pub use amp::Amp;
pub use entities::Event;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AmplitudeError {
    #[error("initialization error")]
    Initialization(String),

    #[error("A network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("unknown error")]
    Unknown,
}


