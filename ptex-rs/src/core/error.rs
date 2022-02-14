#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum OpenError {
    #[error("{0:?}: {1:?}")]
    IOError(std::path::PathBuf, std::string::String),
}
