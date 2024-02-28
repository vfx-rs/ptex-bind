use crate::sys;
use crate::{BorderMode, DataType, EdgeFilterMode, FaceInfo, MeshType};

/// Interface for reading data from a ptex file
///
/// PtexTexture instances can be acquired via the ptexwriter_open() function, or via the
/// PtexCache interface.
///
/// Data access through this interface is returned in v-major order with all data channels
/// interleaved per texel.
pub struct Texture(pub(crate) *mut sys::PtexTexture);

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            sys::ptextexture_release(self.0);
        }
    }
}

impl Texture {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Does the texture contain any in-memory edits?
    pub fn has_edits(&self) -> bool {
        unsafe { sys::ptextexture_has_edits(self.0) }
    }

    /// Does the texture have mip-maps?
    pub fn has_mip_maps(&self) -> bool {
        unsafe { sys::ptextexture_has_mipmaps(self.0) }
    }

    /// Return the alpha channels for the Texture.
    pub fn alpha_channel(&self) -> i32 {
        unsafe { sys::ptextexture_get_alpha_channel(self.0) }
    }

    /// Return the number of channels in the Texture.
    pub fn num_channels(&self) -> i32 {
        unsafe { sys::ptextexture_get_num_channels(self.0) }
    }

    /// Return the number of faces in the Texture.
    pub fn num_faces(&self) -> i32 {
        unsafe { sys::ptextexture_get_num_faces(self.0) }
    }

    /// Return a PathBuf containing the Texture's filename.
    pub fn filename(&self) -> std::path::PathBuf {
        let path_string = unsafe { sys::ptextexture_get_path(self.0) };
        std::path::PathBuf::from(&path_string)
    }

    /// Return the ptex::MeshType for the Texture.
    pub fn mesh_type(&self) -> MeshType {
        MeshType::from(unsafe { sys::ptextexture_get_meshtype(self.0) })
    }

    /// Return the ptex::DataType for the Texture.
    pub fn data_type(&self) -> DataType {
        DataType::from(unsafe { sys::ptextexture_get_datatype(self.0) })
    }

    /// Return the border mode in the U direction.
    pub fn border_mode_u(&self) -> BorderMode {
        BorderMode::from(unsafe { sys::ptextexture_get_border_mode_u(self.0) })
    }

    /// Return the border mode in the U direction.
    pub fn border_mode_v(&self) -> BorderMode {
        BorderMode::from(unsafe { sys::ptextexture_get_border_mode_v(self.0) })
    }

    /// Return the edge filter mode.
    pub fn edge_filter_mode(&self) -> EdgeFilterMode {
        EdgeFilterMode::from(unsafe { sys::ptextexture_get_edge_filter_mode(self.0) })
    }

    /// Access resolution and adjacency information about a face.
    pub fn face_info(&self, face_id: i32) -> FaceInfo {
        FaceInfo(*unsafe { sys::ptextexture_get_face_info(self.0, face_id) })
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
        face_id: i32,
        u: i32,
        v: i32,
        first_channel: i32,
        num_channels: i32,
    ) -> f32 {
        unsafe { sys::ptextexture_get_pixel(self.0, face_id, u, v, first_channel, num_channels) }
    }
}
