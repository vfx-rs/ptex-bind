use crate::sys;

pub type BorderMode = sys::BorderMode;

pub type DataType = sys::DataType;

pub type EdgeFilterMode = sys::EdgeFilterMode;

pub type EdgeId = sys::EdgeId;

pub type MeshType = sys::MeshType;

pub struct Res {
    res: sys::Ptex_Res_t,
}

impl Clone for Res {
    fn clone(&self) -> Self {
        Res::from_value(self.value())
    }
}

impl Into<sys::Ptex_Res_t> for Res {
    fn into(self) -> sys::Ptex_Res_t {
        self.res
    }
}

impl Res {
    fn as_sys_ptr(&self) -> *const sys::Ptex_Res_t {
        std::ptr::addr_of!(self.res)
    }

    pub fn from_uv_log2(u: i8, v: i8) -> Self {
        let res = sys::Ptex_Res_t { ulog2: u, vlog2: v };
        Res { res }
    }

    pub fn from_value(value: u16) -> Self {
        let mut res = sys::Ptex_Res_t { ulog2: 0, vlog2: 0 };
        unsafe {
            sys::Ptex_Res_from_value(std::ptr::addr_of_mut!(res), value);
        }
        Res { res }
    }

    pub fn size(&self) -> usize {
        let mut value: i32 = 0;
        unsafe {
            sys::Ptex_Res_size(self.as_sys_ptr(), &mut value);
        }
        value as usize
    }

    pub fn u(&self) -> i32 {
        let mut value: i32 = 0;
        unsafe {
            sys::Ptex_Res_u(self.as_sys_ptr(), &mut value);
        }
        value
    }

    pub fn v(&self) -> i32 {
        let mut value: i32 = 0;
        unsafe {
            sys::Ptex_Res_v(self.as_sys_ptr(), &mut value);
        }
        value
    }

    pub fn value(&self) -> u16 {
        let mut value: u16 = 0;
        unsafe {
            sys::Ptex_Res_val(self.as_sys_ptr(), std::ptr::addr_of_mut!(value));
        }
        value
    }
}

pub struct FaceInfo {
    face_info: sys::Ptex_FaceInfo_t,
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

    /// Does this face contain constant data?
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
    pub fn from(data_type: crate::DataType) -> f32 {
        let mut value: f32 = 0.0;
        unsafe {
            sys::Ptex_OneValue(std::ptr::addr_of_mut!(value), data_type.into());
        }
        value
    }
}
