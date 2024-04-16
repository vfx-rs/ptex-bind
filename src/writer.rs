use crate::error::Error;
use crate::{f16, sys, DataType, FaceInfo, MeshType, MetaDataType};
use cxx::let_cxx_string;
use std::ffi::CStr;

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

// For buffers convertible to char*.
pub trait AsUInt8Ptr {
    fn as_u8_ptr(&self) -> *const u8;
}

// write_face() accepts a buffer that must be convertable to char*.
pub trait AsFaceData: AsUInt8Ptr {}

// write_meta_data() accepts a buffer that:
//
// 1. Must be convertable to char*.
// 2. Must have a meta data type.
pub trait AsMetaData: AsUInt8Ptr {
    fn meta_data_type(&self) -> MetaDataType;
    fn meta_data_len(&self) -> usize;
}

macro_rules! as_u8_ptr_impl {
    ($typ:ty) => {
        impl AsUInt8Ptr for $typ {
            fn as_u8_ptr(&self) -> *const u8 {
                self.as_ptr() as *const u8
            }
        }
    };
}

macro_rules! as_face_data_impl {
    ($typ:ty) => {
        impl AsFaceData for $typ {}
    };
}

macro_rules! as_meta_data_impl {
    ($typ:ty, $variant:path) => {
        impl AsMetaData for $typ {
            fn meta_data_type(&self) -> MetaDataType {
                $variant
            }
            fn meta_data_len(&self) -> usize {
                self.len()
            }
        }
    };
}

as_u8_ptr_impl!(&[u8]);
as_face_data_impl!(&[u8]);

as_u8_ptr_impl!(&[u16]);
as_face_data_impl!(&[u16]);

as_u8_ptr_impl!(&[f16]);
as_face_data_impl!(&[f16]);

as_u8_ptr_impl!(&[f32]);
as_face_data_impl!(&[f32]);
as_meta_data_impl!(&[f32], MetaDataType::Float);

as_u8_ptr_impl!(Vec<u8>);
as_face_data_impl!(Vec<u8>);

as_u8_ptr_impl!(Vec<u16>);
as_face_data_impl!(Vec<u16>);

as_u8_ptr_impl!(Vec<f16>);
as_face_data_impl!(Vec<f16>);

as_u8_ptr_impl!(Vec<f32>);
as_face_data_impl!(Vec<f32>);
as_meta_data_impl!(Vec<f32>, MetaDataType::Float);

as_u8_ptr_impl!(&[i8]);
as_meta_data_impl!(&[i8], MetaDataType::Int8);
as_u8_ptr_impl!(Vec<i8>);
as_meta_data_impl!(Vec<i8>, MetaDataType::Int8);

as_u8_ptr_impl!(&[i16]);
as_meta_data_impl!(&[i16], MetaDataType::Int16);
as_u8_ptr_impl!(Vec<i16>);
as_meta_data_impl!(Vec<i16>, MetaDataType::Int16);

as_u8_ptr_impl!(&[i32]);
as_meta_data_impl!(&[i32], MetaDataType::Int32);
as_u8_ptr_impl!(Vec<i32>);
as_meta_data_impl!(Vec<i32>, MetaDataType::Int32);

as_u8_ptr_impl!(&[f64]);
as_meta_data_impl!(&[f64], MetaDataType::Double);
as_u8_ptr_impl!(Vec<f64>);
as_meta_data_impl!(Vec<f64>, MetaDataType::Double);

impl AsUInt8Ptr for String {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
}
as_meta_data_impl!(String, MetaDataType::String);

impl AsUInt8Ptr for str {
    fn as_u8_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
}
as_meta_data_impl!(str, MetaDataType::String);

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
                mesh_type,
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
    pub fn write_face<TexelBuf: AsFaceData>(
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

    pub fn write_meta_data<DataBuf: AsMetaData>(&self, key: &CStr, buf: DataBuf) -> bool {
        unsafe {
            sys::ptexwriter_write_meta_data(
                self.0,
                key.as_ptr(),
                buf.meta_data_type(),
                buf.as_u8_ptr(),
                buf.meta_data_len(),
            )
        }
    }
}
