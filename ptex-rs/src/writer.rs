use crate::error::OpenError;
use crate::{DataType,FaceInfo,MeshType};
use crate::sys;

use std::ffi::{CStr,CString};

pub struct Writer(pub(crate) *mut sys::Ptex_PtexWriter_t);

impl Drop for Writer {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                sys::Ptex_PtexWriter_release(self.0);
            }
            self.0 = std::ptr::null_mut();
        }
    }
}

impl Writer {
    pub fn new(
        filename: &std::path::PathBuf,
        mesh_type: MeshType,
        data_type: DataType,
        num_channels: i32,
        alpha_channel: i32,
        num_faces: i32,
        generate_mipmaps: bool,
    ) -> Result<Self, OpenError> {
        let mut error_str = sys::std_string_t::default();
        let mut writer = Writer(std::ptr::null_mut());
        let filename_cstr = CString::new(filename.to_str().unwrap()).unwrap();

        unsafe {
            sys::Ptex_PtexWriter_open(
                std::ptr::addr_of_mut!(writer.0),
                filename_cstr.as_ptr(),
                mesh_type.into(),
                data_type.into(),
                num_channels,
                alpha_channel,
                num_faces,
                std::ptr::addr_of_mut!(error_str),
                generate_mipmaps,
            );
        }

        if writer.0.is_null() {
            unsafe {
                let mut error_ptr: *const i8 = std::ptr::null_mut();
                let _error_msg = sys::std_string_c_str(
                    std::ptr::addr_of_mut!(error_str),
                    std::ptr::addr_of_mut!(error_ptr)
                );

                if error_ptr != std::ptr::null() {
                    let cstr = CStr::from_ptr(error_ptr).to_str().or(Ok("IOError"))?;
                    return Err(
                        OpenError::IOError(filename.to_path_buf(), cstr.to_string())
                    );
                }
            }
            return Err(
                OpenError::IOError(filename.to_path_buf(), "IOError".to_string())
            );
        }

        Ok(writer)
    }


    pub fn write_face_u16(
        &self,
        face_id: i32,
        face_info: &FaceInfo,
        data: &[u16],
        stride: i32,
    ) -> bool
    {
        let mut result = false;
        unsafe {
            sys::Ptex_PtexWriter_writeFace(
                self.0,
                std::ptr::addr_of_mut!(result),
                face_id,
                face_info.as_sys_ptr(),
                std::mem::transmute(data.as_ptr()),
                stride,
            );
        }
        result
    }
}
