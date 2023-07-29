use crate::sys;
//use crate::{sys, Error, Texture};
//use std::ffi::{CStr, CString};

pub struct Cache(*mut sys::PtexCache);

impl Drop for Cache {
    fn drop(&mut self) {
        unsafe {
            sys::ptexcache_release(self.0);
        }
        self.0 = std::ptr::null_mut();
    }
}

impl Cache {
    pub fn new(max_files: i32, max_mem: usize, premultiply: bool) -> Self {
        unsafe { Self(sys::ptexcache_create(max_files, max_mem, premultiply)) }
    }

    /*
    /// Return a mutable pointer to the underlying ptex_sys::Ptex_PtexCache_t.
    fn as_sys_mut_ptr(&self) -> *mut sys::PtexCache {
        self.0
    }

    fn as_sys_mut_ptr_ptr(&mut self) -> *mut *mut sys::PtexCache {
        std::ptr::addr_of_mut!(self.0)
    }
    */

    /*
    /// Return a cached Ptex Reader for the specified filename.
    /// The filename be either an absolute path, a relative path, or a path
    /// relative to the Ptex search path.
    pub fn get(&mut self, filename: &std::path::Path) -> Result<Texture, Error> {
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
                if !error_ptr.is_null() {
                    let cstr = CStr::from_ptr(error_ptr)
                        .to_str()
                        .unwrap_or(&default_error_message);
                    return Err(Error::String(cstr.to_string()));
                }
            }
            return Err(Error::String(default_error_message));
        }

        Ok(texture)
    }
    */

    /// Set the texture search path for a PtexCache.
    pub fn set_search_path(&mut self, path: &str) {
        unsafe {
            sys::ptexcache_set_search_path(self.0, path);
        }
    }

    /// Get the texture search path for a PtexCache.
    pub fn search_path(&self) -> String {
        unsafe { sys::ptexcache_get_search_path(self.0) }
    }
}
