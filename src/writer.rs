use crate::error::Error;
use crate::{f16, sys, DataType, FaceInfo, MeshType};
use cxx::let_cxx_string;

/// Interface for writing data to a ptex file.
///
/// Note: if an alpha channel is specified, then the textures being
/// written to the file are expected to have unmultiplied-alpha data.
/// Generated mipmaps will be premultiplied by the Ptex library.  On
/// read, PtexTexture will (if requested) premultiply all textures by
/// alpha when getData is called; by default only reductions are
/// premultiplied.  If the source textures are already premultiplied,
/// then alphachan can be set to -1 and the library will just leave all
/// the data as-is.  The only reason to store unmultiplied-alpha
/// textures in the file is to preserve the original texture data for
/// later editing.
pub struct Writer(pub(crate) *mut sys::PtexWriter);

impl Drop for Writer {
    fn drop(&mut self) {
        unsafe {
            sys::ptexwriter_release(self.0);
        }
    }
}

// write_face() accepts a buffer that must be convertable to char*.
pub trait AsUInt8Ptr {
    fn as_u8_ptr(&self) -> *const u8;
}

impl AsUInt8Ptr for &[u8] {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_ptr()
    }
}

impl AsUInt8Ptr for &[u16] {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }
}

impl AsUInt8Ptr for &[f16] {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }
}

impl AsUInt8Ptr for &[f32] {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }
}

impl AsUInt8Ptr for Vec<u8> {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_ptr()
    }
}

impl AsUInt8Ptr for Vec<u16> {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }
}

impl AsUInt8Ptr for Vec<f16> {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }
}

impl AsUInt8Ptr for Vec<f32> {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }
}

impl Writer {
    /// Open a new texture file for writing.
    ///
    /// Parameters:
    /// - filename: Path to file.
    /// - mesh_type: Type of mesh for which the textures are defined.
    /// - data_type: Type of data stored within file.
    /// - num_channels:  Number of data channels.
    /// - alpha_channel: alphachan Index of alpha channel, [0..nchannels-1] or -1 if no alpha channel is present.
    /// - num_faces: nfaces Number of faces in mesh.
    /// - genmipmaps: Specify true if mipmaps should be generated.
    pub fn new(
        filename: &std::path::Path,
        mesh_type: MeshType,
        data_type: DataType,
        num_channels: i32,
        alpha_channel: i32,
        num_faces: i32,
        generate_mipmaps: bool,
    ) -> Result<Self, Error> {
        let_cxx_string!(error_str = "");
        let filename_str = filename.to_str().unwrap_or_default();
        let writer = unsafe {
            sys::ptexwriter_open(
                filename_str,
                ptex_sys::MeshType {
                    repr: mesh_type as u32,
                },
                data_type,
                num_channels,
                alpha_channel,
                num_faces,
                generate_mipmaps,
                error_str.as_mut().get_unchecked_mut(),
            )
        };

        if writer.is_null() || !error_str.is_empty() {
            let error_message = if error_str.is_empty() {
                format!("ptex: Writer::new({filename_str}) failed: {error_str}")
            } else {
                format!("ptex: Writer::new({filename_str}) failed")
            };
            return Err(Error::FileIO(filename.to_path_buf(), error_message));
        }

        Ok(Self(writer))
    }

    /// Close the file.  This operation can take some time if mipmaps are being generated or if there
    /// are many edit blocks.  If an error occurs while writing, false is returned and an error string
    /// is written into the error parameter.
    pub fn close(&mut self) -> Result<(), Error> {
        let error_message = unsafe { sys::ptexwriter_close(self.0) };
        if !error_message.is_empty() {
            return Err(Error::Message(error_message));
        }

        Ok(())
    }

    /// Write u8/u16/f16/f32 texture data for a face.
    ///
    /// The data is assumed to be channel-interleaved per texel and stored in v-major order.
    ///
    /// Parameters:
    /// - face_id: Face index [0..nfaces-1].
    /// - face_info: Face resolution and adjacency information.
    /// - texel_buf: Texel data to write.
    /// - stride: Distance between rows, in bytes (if zero, data is assumed packed).
    ///
    /// If an error is encountered while writing, false is returned and an error message can be
    /// retrieved when close is called.
    pub fn write_face<TexelBuf: AsUInt8Ptr>(
        &self,
        face_id: i32,
        face_info: &FaceInfo,
        texel_buf: &TexelBuf,
        stride: i32,
    ) -> bool {
        unsafe {
            sys::ptexwriter_write_face(self.0, face_id, face_info, texel_buf.as_u8_ptr(), stride)
        }
    }
}
