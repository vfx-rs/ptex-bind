use crate::{sys, Error, Texture};
use cxx::let_cxx_string;

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

    /// Return a cached Ptex Reader for the specified filename.
    /// The filename be either an absolute path, a relative path, or a path
    /// relative to the Ptex search path.
    pub fn get(&mut self, filename: &std::path::Path) -> Result<Texture, Error> {
        let_cxx_string!(error_str = "");
        let filename_str = filename.to_string_lossy().to_string();
        let texture = unsafe {
            sys::ptexcache_get(
                self.0,
                filename_str.as_str(),
                error_str.as_mut().get_unchecked_mut(),
            )
        };

        if texture.is_null() {
            let default_error_message = format!("ptex: Cache::get({:?}) failed", filename);
            if error_str.is_empty() {
                return Err(Error::Message(error_str.to_string()));
            }
            return Err(Error::Message(default_error_message));
        }

        Ok(Texture(texture))
    }

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
