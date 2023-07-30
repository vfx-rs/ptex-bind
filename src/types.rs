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
        Self(sys::res_swappeduv(&self.0))
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
    fn from(res: Res) -> sys::Res {
        res.0
    }
}

pub type FaceInfo = sys::FaceInfo;

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
