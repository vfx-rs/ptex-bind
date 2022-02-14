use crate::error::OpenError;
use crate::{DataType,MeshType};
use crate::sys;

use std::ffi::CStr;

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
        let filename_str = filename.to_str().unwrap().as_ptr();

        unsafe {
            sys::Ptex_PtexWriter_open(
                std::ptr::addr_of_mut!(writer.0),
                std::mem::transmute(filename_str),
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
}
