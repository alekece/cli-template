use thiserror::Error;

/// The errors that may occur when using `{{crate_name}}`.
#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Other(#[from] anyhow::Error),
}
