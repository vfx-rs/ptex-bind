#[cxx::bridge(namespace = "Ptex")]
pub mod ffi {
    /// How to handle mesh border when filtering.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum BorderMode {
        /// texel access is clamped to border
        #[cxx_name = "m_clamp"]
        Clamp,
        /// texel beyond border are assumed to be black
        #[cxx_name = "m_black"]
        Black,
        /// texel access wraps to other side of face
        #[cxx_name = "m_periodic"]
        Periodic,
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

    /// How to handle transformation across edges when filtering.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum EdgeFilterMode {
        /// Don't do anything with the values.
        #[cxx_name = "efm_none"]
        None,
        /// Values are vectors in tangent space; rotate values.
        #[cxx_name = "efm_tanvec"]
        TangentVector,
    }

    /// Edge IDs used in adjacency data in the Ptex::FaceInfo struct.
    /// Edge ID usage for triangle meshes is TBD.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum EdgeId {
        /// Bottom edge, from UV (0,0) to (1,0)
        #[cxx_name = "e_bottom"]
        Bottom,
        /// Right edge, from UV (1,0) to (1,1)
        #[cxx_name = "e_right"]
        Right,
        /// Top edge, from UV (1,1) to (0,1)
        #[cxx_name = "e_top"]
        Top,
        /// Left edge, from UV (0,1) to (0,0)
        #[cxx_name = "e_left"]
        Left,
    }

    /// Type of meta data entry.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum MetaDataType {
        /// Null-terminated string.
        #[cxx_name = "mdt_string"]
        String,
        // Signed 8-bit integer.
        #[cxx_name = "mdt_int8"]
        Int8,
        /// Signed 16-bit integer.
        #[cxx_name = "mdt_int16"]
        Int16,
        /// Signed 32-bit integer.
        #[cxx_name = "mdt_int32"]
        Int32,
        /// Single-precision (32-bit) floating point.
        #[cxx_name = "mdt_float"]
        Float,
        /// Double-precision (32-bit) floating point.
        #[cxx_name = "mdt_double"]
        Double,
    }

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

    /// Pixel resolution of a given texture.
    /// The resolution is stored in log form: ulog2 = log2(ures), vlog2 = log2(vres)).
    /// Note: negative ulog2 or vlog2 values are reserved for internal use.
    struct Res {
        /// log base 2 of u resolution, in texels
        ulog2: i8,
        /// log base 2 of v resolution, in texels
        vlog2: i8,
    }

    unsafe extern "C++" {
        include!("Ptexture.h");
        include!("ptex-sys.h");

        type BorderMode;
        type DataType;
        type EdgeFilterMode;
        type MeshType;
        type MetaDataType;
        type PtexTexture;
        type PtexWriter;

        fn size(self: &Res) -> i32;
        fn u(self: &Res) -> i32;
        fn v(self: &Res) -> i32;
        fn val(self: &Res) -> u16;
        fn swappeduv(self: &Res) -> Res;
        fn swapuv(self: &mut Res);
        fn clamp(self: &mut Res, res: &Res);
        fn ntilesu(self: &Res, tileres: Res) -> i32;
        fn ntilesv(self: &Res, tileres: Res) -> i32;
        fn ntiles(self: &Res, tileres: Res) -> i32;

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
