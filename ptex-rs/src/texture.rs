use crate::sys;
use crate::{BorderMode, DataType, EdgeFilterMode, FaceInfo, MeshType};

use std::ffi::CStr;

pub struct Texture(pub(crate) *mut sys::Ptex_PtexTexture_t);

impl Texture {
    pub fn as_sys_mut_ptr(&self) -> *mut sys::Ptex_PtexTexture_t {
        self.0
    }

    pub fn as_sys_mut_ptr_ptr(&mut self) -> *mut *mut sys::Ptex_PtexTexture_t {
        std::ptr::addr_of_mut!(self.0)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Does the texture contain any in-memory edits?
    pub fn has_edits(&self) -> bool {
        let mut edits = false;
        unsafe {
            sys::Ptex_PtexTexture_hasEdits(self.as_sys_mut_ptr(), std::ptr::addr_of_mut!(edits));
        }
        edits
    }

    /// Does the texture have mip-maps?
    pub fn has_mip_maps(&self) -> bool {
        let mut mipmaps = false;
        unsafe {
            sys::Ptex_PtexTexture_hasMipMaps(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(mipmaps),
            );
        }
        mipmaps
    }

    /// Return the alpha channels for the Texture.
    pub fn alpha_channel(&self) -> i32 {
        let mut channel: i32 = 0;
        unsafe {
            sys::Ptex_PtexTexture_alphaChannel(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(channel),
            );
        }
        channel
    }

    /// Return the number of channels in the Texture.
    pub fn num_channels(&self) -> u32 {
        let mut channels: i32 = 0;
        unsafe {
            sys::Ptex_PtexTexture_numChannels(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(channels),
            );
        }
        channels as u32
    }

    /// Return the number of faces in the Texture.
    pub fn num_faces(&self) -> u32 {
        let mut faces: i32 = 0;
        unsafe {
            sys::Ptex_PtexTexture_numFaces(self.as_sys_mut_ptr(), std::ptr::addr_of_mut!(faces));
        }
        faces as u32
    }

    /// Return a PathBuf for the Texture.
    pub fn path(&self) -> std::path::PathBuf {
        let mut path_ptr: *const i8 = std::ptr::null_mut();
        unsafe {
            sys::Ptex_PtexTexture_path(self.as_sys_mut_ptr(), std::ptr::addr_of_mut!(path_ptr));
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
            sys::Ptex_PtexTexture_meshType(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(mesh_type),
            );
        }
        MeshType::from(mesh_type)
    }

    /// Return the ptex::DataType for the Texture.
    pub fn data_type(&self) -> DataType {
        let mut data_type = sys::Ptex_DataType_dt_uint8;
        unsafe {
            sys::Ptex_PtexTexture_dataType(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(data_type),
            );
        }
        DataType::from(data_type)
    }

    /// Return the border mode in the U direction.
    pub fn u_border_mode(&self) -> BorderMode {
        let mut border_mode = sys::Ptex_BorderMode_m_clamp;
        unsafe {
            sys::Ptex_PtexTexture_uBorderMode(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(border_mode),
            );
        }
        BorderMode::from(border_mode)
    }

    /// Return the border mode in the V direction.
    pub fn v_border_mode(&self) -> BorderMode {
        let mut border_mode = sys::Ptex_BorderMode_m_clamp;
        unsafe {
            sys::Ptex_PtexTexture_vBorderMode(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(border_mode),
            );
        }
        BorderMode::from(border_mode)
    }

    /// Return the edge filter mode.
    pub fn edge_filter_mode(&self) -> EdgeFilterMode {
        let mut filter_mode = sys::Ptex_EdgeFilterMode_efm_none;
        unsafe {
            sys::Ptex_PtexTexture_edgeFilterMode(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(filter_mode),
            );
        }
        EdgeFilterMode::from(filter_mode)
    }

    /// Return FaceInfo details for a face.
    pub fn face_info(&self, face_id: u32) -> FaceInfo {
        let info = FaceInfo::new();
        let mut info_sys_ptr = info.as_sys_ptr();
        unsafe {
            sys::Ptex_PtexTexture_getFaceInfo(
                self.as_sys_mut_ptr(),
                std::ptr::addr_of_mut!(info_sys_ptr),
                face_id as i32,
            );
        }
        info
    }

    /// Access a single texel from the highest resolution texture .
    /// The texel data is converted to floating point (integer types
    /// are normalized 0.0 to 1.0).  A subset of the available
    /// channels may be accessed.
    ///
    /// # Parameters
    ///
    /// - `face_id`: Face index [0..num_faces-1]
    /// - `u`: U coordinate [0..ures-1]
    /// - `v`: V coordinate [0..vres-1]
    /// - `first_channel`: First channel to access [0..num_channels-1]
    /// - `num_channels`: Number of channels to access.
    pub fn pixel_f32(
        &self,
        face_id: u32,
        u: u32,
        v: u32,
        first_channel: u32,
        num_channels: u32,
    ) -> f32 {
        let mut result = 0.0;
        unsafe {
            sys::Ptex_PtexTexture_getPixel(
                self.as_sys_mut_ptr(),
                face_id as i32,
                u as i32,
                v as i32,
                std::ptr::addr_of_mut!(result),
                first_channel as i32,
                num_channels as i32,
            );
        }

        result
    }
}
