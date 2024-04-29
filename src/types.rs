use crate::sys;

/// How to handle mesh border when filtering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BorderMode {
    Clamp,
    Black,
    Periodic,
}

/// Convert ptex_sys::BorderMode into BorderMode.
impl From<ptex_sys::BorderMode> for BorderMode {
    fn from(border_mode: ptex_sys::BorderMode) -> BorderMode {
        match border_mode {
            ptex_sys::BorderMode::Clamp => BorderMode::Clamp,
            ptex_sys::BorderMode::Black => BorderMode::Black,
            ptex_sys::BorderMode::Periodic => BorderMode::Periodic,
            _ => BorderMode::Black,
        }
    }
}

/// Convert BorderMode into ptex_sys::BorderMode.
impl From<BorderMode> for ptex_sys::BorderMode {
    fn from(border_mode: BorderMode) -> ptex_sys::BorderMode {
        match border_mode {
            BorderMode::Clamp => ptex_sys::BorderMode::Clamp,
            BorderMode::Black => ptex_sys::BorderMode::Black,
            BorderMode::Periodic => ptex_sys::BorderMode::Periodic,
        }
    }
}

/// Type of data stored in texture file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataType {
    UInt8,
    UInt16,
    Float16,
    Float32,
}

/// Convert ptex_sys::DataType into DataType.
impl From<ptex_sys::DataType> for DataType {
    fn from(data_type: ptex_sys::DataType) -> DataType {
        match data_type {
            ptex_sys::DataType::UInt8 => DataType::UInt8,
            ptex_sys::DataType::UInt16 => DataType::UInt16,
            ptex_sys::DataType::Float16 => DataType::Float16,
            ptex_sys::DataType::Float32 => DataType::Float32,
            _ => DataType::UInt8,
        }
    }
}

/// Convert DataType into ptex_sys::DataType.
impl From<DataType> for ptex_sys::DataType {
    fn from(data_type: DataType) -> ptex_sys::DataType {
        match data_type {
            DataType::UInt8 => ptex_sys::DataType::UInt8,
            DataType::UInt16 => ptex_sys::DataType::UInt16,
            DataType::Float16 => ptex_sys::DataType::Float16,
            DataType::Float32 => ptex_sys::DataType::Float32,
        }
    }
}

/// How to handle transformation across edges when filtering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeFilterMode {
    None,
    TangentVector,
}

/// Convert ptex_sys::EdgeFilterMode into EdgeFilterMode.
impl From<ptex_sys::EdgeFilterMode> for EdgeFilterMode {
    fn from(edge_filter_mode: ptex_sys::EdgeFilterMode) -> EdgeFilterMode {
        match edge_filter_mode {
            ptex_sys::EdgeFilterMode::None => EdgeFilterMode::None,
            ptex_sys::EdgeFilterMode::TangentVector => EdgeFilterMode::TangentVector,
            _ => EdgeFilterMode::None,
        }
    }
}

/// Convert EdgeFilterMode into ptex_sys::EdgeFilterMode.
impl From<EdgeFilterMode> for ptex_sys::EdgeFilterMode {
    fn from(edge_filter_mode: EdgeFilterMode) -> ptex_sys::EdgeFilterMode {
        match edge_filter_mode {
            EdgeFilterMode::None => ptex_sys::EdgeFilterMode::None,
            EdgeFilterMode::TangentVector => ptex_sys::EdgeFilterMode::TangentVector,
        }
    }
}

/// Edge IDs used in adjacency data in the Ptex::FaceInfo struct.
/// Edge ID usage for triangle meshes is TBD.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeId {
    Bottom,
    Right,
    Top,
    Left,
}

/// Convert ptex_sys::EdgeId into EdgeId.
impl From<ptex_sys::EdgeId> for EdgeId {
    fn from(edge_id: ptex_sys::EdgeId) -> EdgeId {
        match edge_id {
            ptex_sys::EdgeId::Bottom => EdgeId::Bottom,
            ptex_sys::EdgeId::Right => EdgeId::Right,
            ptex_sys::EdgeId::Top => EdgeId::Top,
            ptex_sys::EdgeId::Left => EdgeId::Left,
            _ => EdgeId::Bottom,
        }
    }
}

/// Convert EdgeId into ptex_sys::EdgeId.
impl From<EdgeId> for ptex_sys::EdgeId {
    fn from(edge_id: EdgeId) -> ptex_sys::EdgeId {
        match edge_id {
            EdgeId::Bottom => ptex_sys::EdgeId::Bottom,
            EdgeId::Right => ptex_sys::EdgeId::Right,
            EdgeId::Top => ptex_sys::EdgeId::Top,
            EdgeId::Left => ptex_sys::EdgeId::Left,
        }
    }
}

/// Type of base mesh for which the textures are defined.  A mesh
/// can be triangle-based (with triangular textures) or quad-based
/// (with rectangular textures). */
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeshType {
    Quad,
    Triangle,
}

/// Convert ptex_sys::MeshType into MeshType.
impl From<ptex_sys::MeshType> for MeshType {
    fn from(mesh_type: ptex_sys::MeshType) -> MeshType {
        match mesh_type {
            ptex_sys::MeshType::Quad => MeshType::Quad,
            ptex_sys::MeshType::Triangle => MeshType::Triangle,
            _ => MeshType::Quad,
        }
    }
}

/// Convert MeshType into ptex_sys::MeshType.
impl From<MeshType> for ptex_sys::MeshType {
    fn from(mesh_type: MeshType) -> ptex_sys::MeshType {
        match mesh_type {
            MeshType::Quad => ptex_sys::MeshType::Quad,
            MeshType::Triangle => ptex_sys::MeshType::Triangle,
        }
    }
}

/// Type of meta data entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MetaDataType {
    String,
    Int8,
    Int16,
    Int32,
    Float,
    Double,
}

/// Convert ptex_sys::MetaDataType into MetaDataType.
impl From<ptex_sys::MetaDataType> for MetaDataType {
    fn from(meta_data_type: ptex_sys::MetaDataType) -> MetaDataType {
        match meta_data_type {
            ptex_sys::MetaDataType::String => MetaDataType::String,
            ptex_sys::MetaDataType::Int8 => MetaDataType::Int8,
            ptex_sys::MetaDataType::Int16 => MetaDataType::Int16,
            ptex_sys::MetaDataType::Int32 => MetaDataType::Int32,
            ptex_sys::MetaDataType::Float => MetaDataType::Float,
            ptex_sys::MetaDataType::Double => MetaDataType::Double,
            _ => MetaDataType::String,
        }
    }
}

/// Convert MetaDataType into ptex_sys::MetaDataType.
impl From<MetaDataType> for ptex_sys::MetaDataType {
    fn from(meta_data_type: MetaDataType) -> ptex_sys::MetaDataType {
        match meta_data_type {
            MetaDataType::String => ptex_sys::MetaDataType::String,
            MetaDataType::Int8 => ptex_sys::MetaDataType::Int8,
            MetaDataType::Int16 => ptex_sys::MetaDataType::Int16,
            MetaDataType::Int32 => ptex_sys::MetaDataType::Int32,
            MetaDataType::Float => ptex_sys::MetaDataType::Float,
            MetaDataType::Double => ptex_sys::MetaDataType::Double,
        }
    }
}

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
pub struct FaceInfo(pub(crate) sys::FaceInfo);

impl FaceInfo {
    pub fn from_res_and_adjacency<T: Into<Res>>(
        res: T,
        adjacent_faces: &[i32; 4],
        adjacent_edges: &[i32; 4],
        is_subface: bool,
    ) -> Self {
        FaceInfo(ptex_sys::FaceInfo::from_res_and_adjacency(
            res.into(),
            adjacent_faces,
            adjacent_edges,
            is_subface,
        ))
    }

    pub fn resolution(&self) -> Res {
        Res(self.0.resolution())
    }

    pub fn set_resolution<T: Into<Res>>(&mut self, res: T) {
        self.0.set_resolution(res.into())
    }

    pub fn adjacent_edge(&self, edge_id: i32) -> EdgeId {
        EdgeId::from(self.0.adjacent_edge(edge_id))
    }

    pub fn set_adjacent_edges(&mut self, e1: EdgeId, e2: EdgeId, e3: EdgeId, e4: EdgeId) {
        self.0.set_adjacent_edges(
            ptex_sys::EdgeId { repr: e1 as u32 },
            ptex_sys::EdgeId { repr: e2 as u32 },
            ptex_sys::EdgeId { repr: e3 as u32 },
            ptex_sys::EdgeId { repr: e4 as u32 },
        )
    }

    pub fn adjacent_face(&self, face_id: i32) -> i32 {
        self.0.adjacent_face(face_id)
    }

    pub fn set_adjacent_faces(&mut self, f1: i32, f2: i32, f3: i32, f4: i32) {
        self.0.set_adjacent_faces(f1, f2, f3, f4)
    }

    pub fn has_edits(&self) -> bool {
        self.0.has_edits()
    }

    pub fn is_constant(&self) -> bool {
        self.0.is_constant()
    }

    pub fn is_neighborhood_constant(&self) -> bool {
        self.0.is_neighborhood_constant()
    }

    pub fn is_subface(&self) -> bool {
        self.0.is_subface()
    }
}

/// Return the value of "1.0" for the specified DataType (1.0 (float), 255.0 (8bit), ...).
pub struct OneValue;

impl OneValue {
    /// Return the value of "1.0" for the specified DataType (1.0 (float), 255.0 (8bit), ...).
    pub fn get(data_type: crate::DataType) -> f32 {
        sys::one_value(ptex_sys::DataType {
            repr: data_type as u32,
        })
    }

    /// Return the 1.0/value of "1.0" for the specified DataType (1/1.0 (float), 1/255.0 (8bit), ...).
    pub fn get_inverse(data_type: crate::DataType) -> f32 {
        sys::one_value_inverse(ptex_sys::DataType {
            repr: data_type as u32,
        })
    }
}

/// Query the size in bytes for each DataType.
pub struct DataSize;

impl DataSize {
    /// Return the size in bytes for the DataType.
    pub fn get(data_type: crate::DataType) -> i32 {
        sys::data_size(ptex_sys::DataType {
            repr: data_type as u32,
        })
    }
}
