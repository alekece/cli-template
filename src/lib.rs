mod error;

pub use error::Error;

/// A `Result` alias where the `Err` is a `{{crate_name}}::Error`.
pub type Result<T> = std::result::Result<T, Error>;
