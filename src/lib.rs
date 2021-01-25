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
