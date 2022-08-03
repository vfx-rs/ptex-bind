use crate::sys;

use std::ffi::{CStr, CString};

pub struct Cache(pub(crate) *mut sys::Ptex_PtexCache_t);

impl Drop for Cache {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                sys::Ptex_PtexCache_release(self.0);
            }
            self.0 = std::ptr::null_mut();
        }
    }
}

impl Cache {
    pub fn new(max_files: u32, max_mem: usize, premultiply: bool) -> Self {
        let mut cache = Cache(std::ptr::null_mut());
        unsafe {
            sys::Ptex_PtexCache_create(
                std::ptr::addr_of_mut!(cache.0),
                max_files as i32,
                max_mem,
                premultiply,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        }
        cache
    }

    /// Set the Ptex texture search path.
    pub fn set_search_path(&mut self, path: &str) {
        let path_cstr = CString::new(path).unwrap();
        unsafe {
            sys::Ptex_PtexCache_setSearchPath(self.0, path_cstr.as_ptr());
        }
    }

    /// Get the current Ptex texture search path.
    pub fn search_path(&self) -> String {
        let result;
        let c_str;
        let mut c_str_search_path = std::ptr::null();
        unsafe {
            sys::Ptex_PtexCache_getSearchPath(self.0, std::ptr::addr_of_mut!(c_str_search_path));
        }
        if c_str_search_path.is_null() {
            result = String::default();
        } else {
            unsafe {
                c_str = CStr::from_ptr(c_str_search_path)
                    .to_str()
                    .unwrap_or_default();
                result = String::from(c_str);
            }
        }

        result
    }
}
