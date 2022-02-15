#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("{0:?}: {1:?}")]
    FileIO(std::path::PathBuf, std::string::String),

    #[error("{0:?}")]
    String(std::string::String),
}
