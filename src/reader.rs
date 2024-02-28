use crate::{sys, Error, Texture};
use cxx::let_cxx_string;

/// File-handle and memory cache for reading ptex files
///
/// The Cache struct allows cached read access to multiple ptex
/// files while constraining the open file count and memory usage to
/// specified limits.  File and data objects accessed via the cache
/// are added back to the cache when their release method is called.
/// Released objects are maintained in an LRU list and only destroyed
/// when the specified resource limits are exceeded.
///
/// The cache is fully multi-threaded.  Cached data will be shared among
/// all threads that have access to the cache, and the data are protected
/// with internal locks.  See ptex/PtexCache.cpp for details about the caching
/// and locking implementation.
pub struct Cache(*mut sys::PtexCache);

/// Drop implementation for Cache.
impl Drop for Cache {
    fn drop(&mut self) {
        unsafe {
            sys::ptexcache_release(self.0);
        }
    }
}

impl Cache {
    /// Create a cache with the specified limits.
    ///
    /// Parameters:
    /// - max_files: Maximum open file handles.
    ///   If zero, limit is set to 100 open files.
    /// - max_mem:  Maximum allocated memory, in bytes.
    ///   If zero the cache is unlimited.
    /// - premultiply: If true, textures will be premultiplied by
    ///   the alpha channel (if any) when read from disk.  For authoring
    ///   purposes, this should generally be set to false, and for
    ///   rendering purposes, this should generally be set to true.
    ///   See PtexTexture and PtexWriter for more details.
    pub fn new(max_files: i32, max_mem: usize, premultiply: bool) -> Self {
        unsafe { Self(sys::ptexcache_create(max_files, max_mem, premultiply)) }
    }

    /// Return a cached Ptex Reader for the specified filename.
    /// The filename be either an absolute path, a relative path, or a path
    /// relative to the Ptex search path.
    pub fn get<P: AsRef<std::path::Path>>(&mut self, filename: P) -> Result<Texture, Error> {
        let_cxx_string!(error_str = "");
        let filename_str = filename.as_ref().to_string_lossy().to_string();
        let texture = unsafe {
            sys::ptexcache_get(
                self.0,
                filename_str.as_str(),
                error_str.as_mut().get_unchecked_mut(),
            )
        };

        if texture.is_null() {
            let default_error_message = format!("ptex: Cache::get({:?}) failed", filename.as_ref());
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
