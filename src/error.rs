#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("{0:?}: {1:?}")]
    FileIO(std::path::PathBuf, String),

    #[error("{0:?}")]
    Message(String),
}
