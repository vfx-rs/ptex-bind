use crate::sys;

pub type BorderMode = sys::BorderMode;
pub type DataType = sys::DataType;
pub type EdgeFilterMode = sys::EdgeFilterMode;
pub type EdgeId = sys::EdgeId;
pub type MeshType = sys::MeshType;
pub type MetaDataType = sys::MetaDataType;

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
        Self(self.0.swappeduv())
    }

    pub fn size(&self) -> usize {
        self.0.size() as usize
    }

    pub fn u(&self) -> i32 {
        self.0.u()
    }

    pub fn v(&self) -> i32 {
        self.0.v()
    }

    pub fn value(&self) -> u16 {
        self.0.val()
    }

    /// Swap the u and v resolution values in place.
    pub fn swap_uv(&mut self) {
        self.0.swapuv();
    }

    /// Clamp the resolution value against the given value.
    pub fn clamp(&mut self, res: &Res) {
        self.0.clamp(&res.0);
    }

    /// Determine the number of tiles in the u direction for the given tile res.
    pub fn ntilesu(&self, tileres: Res) -> i32 {
        self.0.ntilesu(tileres.0)
    }

    /// Determine the number of tiles in the v direction for the given tile res.
    pub fn ntilesv(&self, tileres: Res) -> i32 {
        self.0.ntilesv(tileres.0)
    }

    /// Determine the total number of tiles for the given tile res.
    pub fn ntiles(&self, tileres: Res) -> i32 {
        self.0.ntiles(tileres.0)
    }
}

impl From<Res> for sys::Res {
    fn from(res: Res) -> sys::Res {
        res.0
    }
}

pub struct FaceInfo(sys::FaceInfo);

impl Default for FaceInfo {
    fn default() -> Self {
        Self(sys::faceinfo_default())
    }
}

impl FaceInfo {
    pub fn from_res_and_adjacency(
        res: Res,
        adjacent_faces: &[i32; 4],
        adjacent_edges: &[i32; 4],
        is_subface: bool,
    ) -> Self {
        Self(sys::faceinfo_from_res_and_adjacency(
            res.0,
            *adjacent_faces,
            *adjacent_edges,
            is_subface,
        ))
    }

    /// Does this face have edits?
    pub fn has_edits(&self) -> bool {
        self.0.has_edits()
    }

    /// Does this face contain constant data?
    pub fn is_constant(&self) -> bool {
        self.0.is_constant()
    }

    /// Does this face's neighborhood contain constant data?
    pub fn is_neighborhood_constant(&self) -> bool {
        self.0.is_neighborhood_constant()
    }

    /// Is this logical face a subface?
    pub fn is_subface(&self) -> bool {
        self.0.is_subface()
    }

    /// Get the adjacent edge for this face. The edge_id must be 0..3.
    pub fn adjacent_edge(&self, edge_id: i32) -> EdgeId {
        self.0.adjacent_edge(edge_id)
    }

    /// Get the adjacent face for this face. The edge_id must be 0..3.
    pub fn adjacent_face(&self, edge_id: i32) -> i32 {
        self.0.adjacent_face(edge_id)
    }

    /// Set the adjacent faces for a face.
    pub fn set_adjacent_faces(&mut self, f1: i32, f2: i32, f3: i32, f4: i32) {
        self.0.set_adjacent_faces(f1, f2, f3, f4);
    }

    /// Set the adjacent edges for a face.
    pub fn set_adjacent_edges(&mut self, e1: i32, e2: i32, e3: i32, e4: i32) {
        self.0.set_adjacent_edges(e1, e2, e3, e4);
    }

    /// Return a Res resolution struct.
    pub fn resolution(&self) -> Res {
        Res(self.0.res)
    }

    /// Set the resolution for this face.
    pub fn set_resolution(&mut self, res: Res) {
        self.0.res = res.into();
    }
}

/// Return the value of "1.0" for the specified DataType (1.0 (float), 255.0 (8bit), ...).
pub struct OneValue;

impl OneValue {
    pub fn get(data_type: crate::DataType) -> f32 {
        sys::one_value(data_type)
    }

    pub fn get_inverse(data_type: crate::DataType) -> f32 {
        sys::one_value_inverse(data_type)
    }
}

/// Return the size-in-bytes for the specified DataType.
pub struct DataSize;

impl DataSize {
    pub fn get(data_type: crate::DataType) -> i32 {
        sys::data_size(data_type)
    }
}
