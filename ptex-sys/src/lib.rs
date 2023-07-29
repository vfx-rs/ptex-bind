#[cxx::bridge(namespace = "Ptex")]
pub mod ffi {
    /// How to handle mesh border when filtering.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    /// Information about a face, as stored in the Ptex file header.
    /// The FaceInfo data contains the face resolution and neighboring face
    /// adjacency information as well as a set of flags describing the face.
    ///
    /// The adjfaces data member contains the face ids of the four neighboring faces.
    /// The neighbors are accessed in EdgeId order, CCW, starting with the bottom edge.
    /// The adjedges data member contains the corresponding edge id for each neighboring face.
    ///
    /// If a face has no neighbor for a given edge, the adjface id should be -1, and the
    /// adjedge id doesn't matter (but is typically zero).
    ///
    /// If an adjacent face is a pair of subfaces, the id of the first subface as encountered
    /// in a CCW traversal should be stored as the adjface id.
    struct FaceInfo {
        /// Resolution of face.
        res: Res,
        ///< Adjacent edges, 2 bits per edge.
        adjedges: u8,
        /// Flags.
        flags: u8,
        /// Adjacent faces (-1 == no adjacent face).
        adjfaces: [u32; 4],
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

        // struct Res
        #[namespace = "Ptex::sys"]
        fn res_default() -> Res;

        #[namespace = "Ptex::sys"]
        fn res_from_uv(u: i8, v: i8) -> Res;

        #[namespace = "Ptex::sys"]
        fn res_from_value(value: u16) -> Res;

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

        // struct FaceInfo
        #[namespace = "Ptex::sys"]
        fn faceinfo_default() -> FaceInfo;

        #[namespace = "Ptex::sys"]
        fn faceinfo_from_res(res: Res) -> FaceInfo;

        #[namespace = "Ptex::sys"]
        fn faceinfo_from_res_and_adjacency(
            res: Res,
            adjacent_faces: [i32; 4],
            adjacent_edges: [i32; 4],
            is_subface: bool,
        ) -> FaceInfo;

        #[cxx_name = "adjedge"]
        fn adjacent_edge(self: &FaceInfo, edge_id: i32) -> EdgeId;

        #[cxx_name = "adjface"]
        fn adjacent_face(self: &FaceInfo, edge_id: i32) -> i32;

        #[cxx_name = "isConstant"]
        fn is_constant(self: &FaceInfo) -> bool;

        #[cxx_name = "isNeighborhoodConstant"]
        fn is_neighborhood_constant(self: &FaceInfo) -> bool;

        #[cxx_name = "hasEdits"]
        fn has_edits(self: &FaceInfo) -> bool;

        #[cxx_name = "isSubface"]
        fn is_subface(self: &FaceInfo) -> bool;

        #[cxx_name = "setadjfaces"]
        fn set_adjacent_faces(self: &mut FaceInfo, f1: i32, f2: i32, f3: i32, f4: i32);

        #[cxx_name = "setadjedges"]
        fn set_adjacent_edges(self: &mut FaceInfo, e1: i32, e2: i32, e3: i32, e4: i32);

        #[cxx_name = "OneValue"]
        fn one_value(data_type: DataType) -> f32;

        #[cxx_name = "OneValueInv"]
        fn one_value_inverse(data_type: DataType) -> f32;

        #[cxx_name = "DataSize"]
        fn data_size(data_type: DataType) -> i32;

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

impl Copy for Res {}

impl Clone for Res {
    fn clone(&self) -> Self {
        Res {
            ulog2: self.ulog2,
            vlog2: self.vlog2,
        }
    }
}
impl std::fmt::Debug for Res {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Res")
            .field("ulog2", &self.ulog2)
            .field("vlog2", &self.vlog2)
            .finish()
    }
}

impl Default for Res {
    fn default() -> Self {
        ffi::res_default()
    }
}

impl Eq for Res {}

impl PartialEq for Res {
    fn eq(&self, res: &ffi::Res) -> bool {
        self.ulog2 == res.ulog2 && self.vlog2 == res.vlog2
    }
}

pub use ffi::*;
