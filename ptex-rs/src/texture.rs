use crate::sys;
use crate::{BorderMode, DataType, EdgeFilterMode, FaceInfo, MeshType};

use std::ffi::CStr;

pub struct Texture(pub(crate) *mut sys::Ptex_PtexTexture_t);

impl Texture {
    pub fn as_mut_ptr(&mut self) -> *mut *mut sys::Ptex_PtexTexture_t {
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

    /// Return FaceInfo details for a face.
    pub fn face_info(&self, face_id: u32) -> FaceInfo {
        let info = FaceInfo::new();
        let mut info_sys_ptr = info.as_sys_ptr();
        unsafe {
            sys::Ptex_PtexTexture_getFaceInfo(self.0, std::ptr::addr_of_mut!(info_sys_ptr), face_id as i32);
        }
        info
    }
}
