use crate::core;
use crate::core::OpenError;
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
        mesh_type: core::MeshType,
        data_type: core::DataType,
    ) -> Result<Self, OpenError> {
        let alpha_channel = -1;
        let num_channels = 3;
        let num_faces = 9;
        let genmipmaps = false;

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
                genmipmaps,
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
