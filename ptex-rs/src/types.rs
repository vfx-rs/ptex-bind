use crate::sys;


pub struct Res {
    res: sys::Ptex_Res_t,
}


impl Into<sys::Ptex_Res_t> for Res {
    fn into(self) -> sys::Ptex_Res_t {
        self.res
    }
}


impl Res {
    pub fn from_uv_log2(u: i8, v: i8) -> Self {
        let mut res = sys::Ptex_Res_t::default();
        unsafe {
            sys::Ptex_Res_from_uv_log2(std::ptr::addr_of_mut!(res), u, v);
        }
        Res { res }
    }

    pub fn from_value(value: u16) -> Self {
        let mut res = sys::Ptex_Res_t::default();
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
