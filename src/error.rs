/// The main Error type returns either error Messages or FileIO errors.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    /// An error occurred when reading from a Pathbuf.
    #[error("{0:?}: {1:?}")]
    FileIO(std::path::PathBuf, String),

    /// General error messages.
    #[error("{0:?}")]
    Message(String),

    #[error("{0}")]
    EnumConversion(#[from] crate::types::EnumConversionError),
}
