use crate::error::Error;
use crate::sys;

use std::ffi::{CStr, CString};

pub struct Cache(pub(crate) *mut sys::Ptex_PtexCache_t);

pub struct Texture(pub(crate) *mut sys::Ptex_PtexTexture_t);

pub type MeshType = sys::MeshType;

pub type DataType = sys::DataType;

pub type BorderMode = sys::BorderMode;

pub type EdgeFilterMode = sys::EdgeFilterMode;

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
                cache.as_mut_ptr(),
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
    fn as_mut_ptr(&mut self) -> *mut *mut sys::Ptex_PtexCache_t {
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
                self.0,
                texture.as_mut_ptr(),
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

impl Texture {
    fn as_mut_ptr(&mut self) -> *mut *mut sys::Ptex_PtexTexture_t {
        std::ptr::addr_of_mut!(self.0)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Does the texture contain any in-memory edits?
    pub fn has_edits(&self) -> bool {
        let mut edits = false;
        unsafe {
            sys::Ptex_PtexTexture_hasEdits(self.0, std::ptr::addr_of_mut!(edits));
        }
        edits
    }

    /// Does the texture have mip-maps?
    pub fn has_mip_maps(&self) -> bool {
        let mut mipmaps = false;
        unsafe {
            sys::Ptex_PtexTexture_hasMipMaps(self.0, std::ptr::addr_of_mut!(mipmaps));
        }
        mipmaps
    }

    /// Return the alpha channels for the Texture.
    pub fn alpha_channel(&self) -> i32 {
        let mut channel: i32 = 0;
        unsafe {
            sys::Ptex_PtexTexture_alphaChannel(self.0, std::ptr::addr_of_mut!(channel));
        }
        channel
    }

    /// Return the number of channels in the Texture.
    pub fn num_channels(&self) -> u32 {
        let mut channels: i32 = 0;
        unsafe {
            sys::Ptex_PtexTexture_numChannels(self.0, std::ptr::addr_of_mut!(channels));
        }
        channels as u32
    }

    /// Return the number of faces in the Texture.
    pub fn num_faces(&self) -> u32 {
        let mut faces: i32 = 0;
        unsafe {
            sys::Ptex_PtexTexture_numFaces(self.0, std::ptr::addr_of_mut!(faces));
        }
        faces as u32
    }

    /// Return a PathBuf for the Texture.
    pub fn path(&self) -> std::path::PathBuf {
        let mut path_ptr: *const i8 = std::ptr::null_mut();
        unsafe {
            sys::Ptex_PtexTexture_path(self.0, std::ptr::addr_of_mut!(path_ptr));
        }

        if !path_ptr.is_null() {
            let path_cstr = unsafe { CStr::from_ptr(path_ptr).to_str().unwrap_or_default() };
            std::path::PathBuf::from(path_cstr)
        } else {
            std::path::PathBuf::default()
        }
    }

    /// Return the ptex::MeshType for the Texture.
    pub fn mesh_type(&self) -> MeshType {
        let mut mesh_type = sys::Ptex_MeshType_mt_triangle;
        unsafe {
            sys::Ptex_PtexTexture_meshType(self.0, std::ptr::addr_of_mut!(mesh_type));
        }
        MeshType::from(mesh_type)
    }

    /// Return the ptex::DataType for the Texture.
    pub fn data_type(&self) -> DataType {
        let mut data_type = sys::Ptex_DataType_dt_uint8;
        unsafe {
            sys::Ptex_PtexTexture_dataType(self.0, std::ptr::addr_of_mut!(data_type));
        }
        DataType::from(data_type)
    }

    /// Return the border mode in the U direction.
    pub fn u_border_mode(&self) -> BorderMode {
        let mut border_mode = sys::Ptex_BorderMode_m_clamp;
        unsafe {
            sys::Ptex_PtexTexture_uBorderMode(self.0, std::ptr::addr_of_mut!(border_mode));
        }
        BorderMode::from(border_mode)
    }

    /// Return the border mode in the V direction.
    pub fn v_border_mode(&self) -> BorderMode {
        let mut border_mode = sys::Ptex_BorderMode_m_clamp;
        unsafe {
            sys::Ptex_PtexTexture_vBorderMode(self.0, std::ptr::addr_of_mut!(border_mode));
        }
        BorderMode::from(border_mode)
    }

    /// Return the edge filter mode.
    pub fn edge_filter_mode(&self) -> EdgeFilterMode {
        let mut filter_mode = sys::Ptex_EdgeFilterMode_efm_none;
        unsafe {
            sys::Ptex_PtexTexture_edgeFilterMode(self.0, std::ptr::addr_of_mut!(filter_mode));
        }
        EdgeFilterMode::from(filter_mode)
    }
}
