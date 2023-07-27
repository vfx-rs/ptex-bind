#[cxx::bridge(namespace = "Ptex")]
pub mod ffi {
    /// Type of base mesh for which the textures are defined.  A mesh
    /// can be triangle-based (with triangular textures) or quad-based
    /// (with rectangular textures). */
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum MeshType {
        /// Mesh is triangle-based.
        #[cxx_name = "mt_triangle"]
        Triangle,
        /// Mesh is quad-based.
        #[cxx_name = "mt_quad"]
        Quad,
    }

    /// Type of data stored in texture file.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum DataType {
        /// Unsigned, 8-bit integer.
        #[cxx_name = "dt_uint8"]
        UInt8,
        /// Unsigned, 16-bit integer.
        #[cxx_name = "dt_uint16"]
        UInt16,
        /// Half-precision (16-bit) floating point.
        #[cxx_name = "dt_half"]
        Half,
        /// Single-precision (32-bit) floating point.
        #[cxx_name = "dt_float"]
        Float,
    }

    unsafe extern "C++" {
        include!("Ptexture.h");
        include!("ptex-sys.h");

        type MeshType;
        type DataType;
        type PtexTexture;
        type PtexWriter;

        /// Create a PtexWriter.
        ///
        /// # Safety
        /// Should not be called outside of the ptex::ffi::sys crate.
        #[allow(clippy::too_many_arguments)]
        #[namespace = "Ptex::sys"]
        unsafe fn writer_open(
            filename: &str,
            meshtype: MeshType,
            datatype: DataType,
            num_channels: i32,
            alpha_channel: i32,
            num_faces: i32,
            genmipmaps: bool,
            error_str: *mut CxxString,
        ) -> *mut PtexWriter;
    }
}

pub use ffi::*;
