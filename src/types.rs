use crate::sys;

/// How to handle mesh border when filtering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum BorderMode {
    Clamp = ptex_sys::BorderMode::Clamp.repr,
    Black = ptex_sys::BorderMode::Black.repr,
    Periodic = ptex_sys::BorderMode::Periodic.repr,
}

impl From<ptex_sys::BorderMode> for BorderMode {
    fn from(border_mode: ptex_sys::BorderMode) -> BorderMode {
        match border_mode {
            ptex_sys::BorderMode::Clamp => BorderMode::Clamp,
            ptex_sys::BorderMode::Black => BorderMode::Black,
            ptex_sys::BorderMode::Periodic => BorderMode::Periodic,
            _ => panic!("Unsupported border mode"),
        }
    }
}

/// Type of data stored in texture file.
pub type DataType = sys::DataType;

/// How to handle transformation across edges when filtering.
pub type EdgeFilterMode = sys::EdgeFilterMode;

/// Edge IDs used in adjacency data in the Ptex::FaceInfo struct.
/// Edge ID usage for triangle meshes is TBD.
pub type EdgeId = sys::EdgeId;

/// Type of base mesh for which the textures are defined.  A mesh
/// can be triangle-based (with triangular textures) or quad-based
/// (with rectangular textures). */
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeshType {
    Quad = ptex_sys::MeshType::Quad.repr,
    Triangle = ptex_sys::MeshType::Triangle.repr,
}

impl From<ptex_sys::MeshType> for MeshType {
    fn from(mesh_type: ptex_sys::MeshType) -> MeshType {
        match mesh_type {
            ptex_sys::MeshType::Quad => MeshType::Quad,
            ptex_sys::MeshType::Triangle => MeshType::Triangle,
            _ => {
                panic!("Unrecognized meshtype")
            }
        }
    }
}

/// Type of meta data entry.
pub type MetaDataType = sys::MetaDataType;

/// Pixel resolution of a given texture.
/// The resolution is stored in log form: ulog2 = log2(ures), vlog2 = log2(vres)).
/// Note: negative ulog2 or vlog2 values are reserved for internal use.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Res(sys::Res);

impl Res {
    pub fn from_uv(u: i8, v: i8) -> Self {
        Self(sys::res_from_uv(u, v))
    }

    pub fn from_value(value: u16) -> Self {
        Self(sys::res_from_value(value))
    }
    /// Get value of resolution with u and v swapped.
    pub fn clone_swapped(&self) -> Self {
        Self(sys::res_swappeduv(&self.0))
    }

    /// Total size of specified texture in texels (u * v).
    pub fn size(&self) -> usize {
        self.0.size() as usize
    }

    /// U resolution in texels.
    pub fn u(&self) -> i32 {
        self.0.u()
    }

    /// V resolution in texels.
    pub fn v(&self) -> i32 {
        self.0.v()
    }

    /// Resolution as a single 16-bit integer value.
    pub fn value(&self) -> u16 {
        self.0.val()
    }

    /// Swap the u and v resolution values in place.
    pub fn swap_uv(&mut self) {
        sys::res_swapuv(&mut self.0);
    }

    /// Clamp the resolution value against the given value.
    pub fn clamp(&mut self, res: Res) {
        sys::res_clamp(&mut self.0, &res.0);
    }

    /// Determine the number of tiles in the u direction for the given tile res.
    pub fn ntilesu(&self, tileres: Res) -> i32 {
        sys::res_ntilesu(&self.0, tileres.0)
    }

    /// Determine the number of tiles in the v direction for the given tile res.
    pub fn ntilesv(&self, tileres: Res) -> i32 {
        sys::res_ntilesv(&self.0, tileres.0)
    }

    /// Determine the total number of tiles for the given tile res.
    pub fn ntiles(&self, tileres: Res) -> i32 {
        sys::res_ntiles(&self.0, tileres.0)
    }
}

impl From<Res> for sys::Res {
    /// Res can be used in place of a sys::Res.
    fn from(res: Res) -> sys::Res {
        res.0
    }
}

/// Information about a face, as stored in the Ptex file header.
///
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
pub type FaceInfo = sys::FaceInfo;

/// Return the value of "1.0" for the specified DataType (1.0 (float), 255.0 (8bit), ...).
pub struct OneValue;

impl OneValue {
    /// Return the value of "1.0" for the specified DataType (1.0 (float), 255.0 (8bit), ...).
    pub fn get(data_type: crate::DataType) -> f32 {
        sys::one_value(data_type)
    }

    /// Return the 1.0/value of "1.0" for the specified DataType (1/1.0 (float), 1/255.0 (8bit), ...).
    pub fn get_inverse(data_type: crate::DataType) -> f32 {
        sys::one_value_inverse(data_type)
    }
}

/// Query the size in bytes for each DataType.
pub struct DataSize;

impl DataSize {
    /// Return the size in bytes for the DataType.
    pub fn get(data_type: crate::DataType) -> i32 {
        sys::data_size(data_type)
    }
}
