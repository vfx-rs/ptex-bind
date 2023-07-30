/// Low-level ptex-sys FFI bindings to Ptex.
use ptex_sys as sys;

/// Errors returned by the ptex crate.
mod error;
pub use error::Error;

/// Core Ptex data types.
mod types;
pub use types::BorderMode;
pub use types::DataSize;
pub use types::DataType;
pub use types::EdgeFilterMode;
pub use types::EdgeId;
pub use types::FaceInfo;
pub use types::MeshType;
pub use types::MetaDataType;
pub use types::OneValue;
pub use types::Res;

/// Read Ptex files.
mod reader;
pub use reader::Cache;

/// Write Ptex files.
mod writer;
pub use writer::Writer;

/// Access texture data for Ptex files.
mod texture;
pub use texture::Texture;
