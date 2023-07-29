use ptex_sys as sys;

mod error;
pub use error::Error;

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

mod reader;
pub use reader::Cache;
//mod writer;
//pub use writer::Writer;
mod texture;
pub use texture::Texture;
