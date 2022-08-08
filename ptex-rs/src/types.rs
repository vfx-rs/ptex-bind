use crate::sys;

pub type MeshType = sys::MeshType;

pub type DataType = sys::DataType;

pub type BorderMode = sys::BorderMode;

pub type EdgeFilterMode = sys::EdgeFilterMode;

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
            sys::Ptex_Res_size(std::ptr::addr_of!(self.res), &mut value);
        }
        value as usize
    }

    pub fn u(&self) -> i32 {
        let mut value: i32 = 0;
        unsafe {
            sys::Ptex_Res_u(std::ptr::addr_of!(self.res), &mut value);
        }
        value
    }

    pub fn v(&self) -> i32 {
        let mut value: i32 = 0;
        unsafe {
            sys::Ptex_Res_v(std::ptr::addr_of!(self.res), &mut value);
        }
        value
    }

    pub fn value(&self) -> u16 {
        let mut value: u16 = 0;
        unsafe {
            sys::Ptex_Res_val(std::ptr::addr_of!(self.res), std::ptr::addr_of_mut!(value));
        }
        value
    }
}

pub struct FaceInfo {
    face_info: sys::Ptex_FaceInfo_t,
}

impl FaceInfo {
    pub fn as_sys_ptr(&self) -> *const sys::Ptex_FaceInfo_t {
        std::ptr::addr_of!(self.face_info)
    }
}

impl FaceInfo {
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
