
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unable to open for writing: {0:?}")]
    OpenWrite(std::path::PathBuf),
}
