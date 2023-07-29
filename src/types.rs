use crate::sys;

pub type BorderMode = sys::BorderMode;
pub type DataType = sys::DataType;
pub type EdgeFilterMode = sys::EdgeFilterMode;
pub type EdgeId = sys::EdgeId;
pub type MeshType = sys::MeshType;
pub type MetaDataType = sys::MetaDataType;

pub struct Res {
    res: sys::Res,
}

impl Res {
    pub fn from_uv_log2(u: i8, v: i8) -> Self {
        Res {
            res: sys::Res { ulog2: u, vlog2: v },
        }
    }

    pub fn from_value(value: u16) -> Self {
        Self::from_uv_log2((value & 0xff) as i8, ((value >> 8) & 0xff) as i8)
    }

    pub fn size(&self) -> usize {
        self.res.size() as usize
    }

    pub fn u(&self) -> i32 {
        self.res.u()
    }

    pub fn v(&self) -> i32 {
        self.res.v()
    }

    pub fn value(&self) -> u16 {
        self.res.val()
    }

    /// Get value of resolution with u and v swapped.
    pub fn swappeduv(&self) -> Self {
        Res { res: self.res.swappeduv() }
    }

    /// Swap the u and v resolution values in place.
    pub fn swapuv(&mut self) {
        self.res.swapuv()
    }

    /// Clamp the resolution value against the given value.
    pub fn clamp(&mut self, res: &Res) {
        self.res.clamp(&res.res);
    }

    /// Determine the number of tiles in the u direction for the given tile res.
    fn ntilesu(&self, tileres: Res) -> i32 {
        self.res.ntilesu(tileres.res)
    }

    /// Determine the number of tiles in the v direction for the given tile res.
    fn ntilesv(&self, tileres: Res) -> i32 {
        self.res.ntilesv(tileres.res)
    }

    /// Determine the total number of tiles for the given tile res.
    fn ntiles(&self, tileres: Res) -> i32 {
        self.res.ntiles(tileres.res)
    }
}

impl Clone for Res {
    fn clone(&self) -> Self {
        Res::from_value(self.value())
    }
}

impl From<Res> for sys::Res {
    fn from(res: Res) -> sys::Res {
        res.res
    }
}

/*
pub struct FaceInfo {
    face_info: sys::Ptex_FaceInfo_t,
}

impl Default for FaceInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl FaceInfo {
    pub fn new() -> Self {
        FaceInfo {
            face_info: sys::Ptex_FaceInfo_t::default(),
        }
    }

    pub fn from_res_and_adjacency(
        res: &Res,
        adjacent_faces: &[i32],
        adjacent_edges: &[i32],
        is_subface: bool,
    ) -> Self {
        let mut face_info = sys::Ptex_FaceInfo_t::default();
        unsafe {
            sys::Ptex_FaceInfo_from_res_and_adjacency(
                std::ptr::addr_of_mut!(face_info),
                res.clone().into(),
                std::mem::transmute(adjacent_faces.as_ptr()),
                std::mem::transmute(adjacent_edges.as_ptr()),
                is_subface,
            );
        }

        FaceInfo { face_info }
    }

    pub fn as_sys_ptr(&self) -> *const sys::Ptex_FaceInfo_t {
        std::ptr::addr_of!(self.face_info)
    }

    pub fn as_sys_mut_ptr(&mut self) -> *mut sys::Ptex_FaceInfo_t {
        std::ptr::addr_of_mut!(self.face_info)
    }

    /// Does this face have edits?
    pub fn has_edits(&self) -> bool {
        let mut result = false;
        unsafe {
            sys::Ptex_FaceInfo_hasEdits(self.as_sys_ptr(), std::ptr::addr_of_mut!(result));
        }

        result
    }

    /// Does this face contain constant data?
    pub fn is_constant(&self) -> bool {
        let mut result = false;
        unsafe {
            sys::Ptex_FaceInfo_isConstant(self.as_sys_ptr(), std::ptr::addr_of_mut!(result));
        }

        result
    }

    /// Does this face's neighborhood contain constant data?
    pub fn is_neighborhood_constant(&self) -> bool {
        let mut result = false;
        unsafe {
            sys::Ptex_FaceInfo_isNeighborhoodConstant(
                self.as_sys_ptr(),
                std::ptr::addr_of_mut!(result),
            );
        }

        result
    }

    /// Is this logical face a subface?
    pub fn is_subface(&self) -> bool {
        let mut result = false;
        unsafe {
            sys::Ptex_FaceInfo_isSubface(self.as_sys_ptr(), std::ptr::addr_of_mut!(result));
        }

        result
    }

    /// Get the adjacent edge for this face. The edge_id must be 0..3.
    pub fn adjacent_edge(&self, edge_id: i32) -> EdgeId {
        let mut sys_edge_id = sys::Ptex_EdgeId_e_bottom;
        unsafe {
            sys::Ptex_FaceInfo_adjedge(
                self.as_sys_ptr(),
                std::ptr::addr_of_mut!(sys_edge_id),
                std::cmp::min(std::cmp::max(edge_id, 0), 3),
            );
        }

        EdgeId::from(sys_edge_id)
    }

    /// Get the adjacent face for this face. The edge_id must be 0..3.
    pub fn adjacent_face(&self, edge_id: i32) -> i32 {
        let mut face_id: i32 = -1;
        unsafe {
            sys::Ptex_FaceInfo_adjface(
                self.as_sys_ptr(),
                std::ptr::addr_of_mut!(face_id),
                std::cmp::min(std::cmp::max(edge_id, 0), 3),
            );
        }

        face_id
    }

    /// Set the adjacent faces for a face.
    pub fn set_adjacent_faces(&mut self, f1: i32, f2: i32, f3: i32, f4: i32) {
        unsafe {
            sys::Ptex_FaceInfo_setadjfaces(self.as_sys_mut_ptr(), f1, f2, f3, f4);
        }
    }

    /// Set the adjacent edges for a face.
    pub fn set_adjacent_edges(&mut self, f1: EdgeId, f2: EdgeId, f3: EdgeId, f4: EdgeId) {
        unsafe {
            sys::Ptex_FaceInfo_setadjedges(
                self.as_sys_mut_ptr(),
                f1 as i32,
                f2 as i32,
                f3 as i32,
                f4 as i32,
            );
        }
    }

    /// Return a Res resolution struct.
    pub fn resolution(&self) -> Res {
        let mut res_addr: *const sys::Ptex_Res_t = std::ptr::null_mut();
        unsafe {
            sys::Ptex_FaceInfo_get_res(self.as_sys_ptr(), std::ptr::addr_of_mut!(res_addr));
        }
        match res_addr.is_null() {
            true => Res::from_value(0),
            false => unsafe {
                Res {
                    res: (*res_addr).clone(),
                }
            },
        }
    }

    /// Set the resolution for this face.
    pub fn set_resolution(&mut self, res: &Res) {
        unsafe {
            sys::Ptex_FaceInfo_set_res(self.as_sys_mut_ptr(), res.as_sys_ptr());
        }
    }
}

pub struct OneValue;

impl OneValue {
    pub fn get(data_type: crate::DataType) -> f32 {
        let mut value: f32 = 0.0;
        unsafe {
            sys::Ptex_OneValue(std::ptr::addr_of_mut!(value), data_type.into());
        }
        value
    }
}

*/
