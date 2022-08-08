use crate::{sys, Error, Texture};

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
                cache.as_sys_mut_ptr_ptr(),
                max_files as i32,
                max_mem,
                premultiply,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        }
        cache
    }

    /// Return a mutable pointer to the underlying ptex_sys::Ptex_PtexCache_t.
    fn as_sys_mut_ptr(&self) -> *mut sys::Ptex_PtexCache_t {
        self.0
    }

    fn as_sys_mut_ptr_ptr(&mut self) -> *mut *mut sys::Ptex_PtexCache_t {
        std::ptr::addr_of_mut!(self.0)
    }

    /// Return a cached Ptex Reader for the specified filename.
    /// The filename be either an absolute path, a relative path, or a path
    /// relative to the Ptex search path.
    pub fn get(&mut self, filename: &std::path::PathBuf) -> Result<Texture, Error> {
        let mut texture = Texture(std::ptr::null_mut());
        let mut error_str = sys::std_string_t::default();
        let filename_cstr = CString::new(filename.to_string_lossy().as_ref()).unwrap_or_default();
        unsafe {
            sys::Ptex_PtexCache_get(
                self.as_sys_mut_ptr(),
                texture.as_sys_mut_ptr_ptr(),
                filename_cstr.as_ptr(),
                std::ptr::addr_of_mut!(error_str),
            );
        }

        if texture.is_null() {
            let default_error_message = format!("ptex: Cache::get({:?}) failed", filename);
            let mut error_ptr: *const i8 = std::ptr::null_mut();
            unsafe {
                let _error_msg = sys::std_string_c_str(
                    std::ptr::addr_of_mut!(error_str),
                    std::ptr::addr_of_mut!(error_ptr),
                );
                if error_ptr != std::ptr::null() {
                    let cstr = CStr::from_ptr(error_ptr)
                        .to_str()
                        .unwrap_or(&default_error_message);
                    return Err(Error::String(cstr.to_string()));
                }
            }
            return Err(Error::String(default_error_message.to_string()));
        }

        Ok(texture)
    }

    /// Set the Ptex texture search path.
    pub fn set_search_path(&mut self, path: &str) {
        let path_cstr = CString::new(path).unwrap_or_default();
        unsafe {
            sys::Ptex_PtexCache_setSearchPath(self.as_sys_mut_ptr(), path_cstr.as_ptr());
        }
    }

    /// Get the current Ptex texture search path.
    pub fn search_path(&self) -> String {
        let result;
        let c_str;
        let mut c_str_search_path = std::ptr::null();
        unsafe {
            sys::Ptex_PtexCache_getSearchPath(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(c_str_search_path),
            );
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
