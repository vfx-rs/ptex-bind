use crate::error::Error;
use crate::sys;
use crate::{DataType, MeshType};
use cxx::let_cxx_string;

pub struct Writer(pub(crate) *mut sys::PtexWriter);

impl Drop for Writer {
    fn drop(&mut self) {
        unsafe {
            sys::ptexwriter_release(self.0);
        }
    }
}

impl Writer {
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

    /*
        pub fn close(&self) -> Result<(), Error> {
            let mut result = false;
            let mut error_str = sys::std_string_t::default();
            if self.0.is_null() {
                return Ok(());
            }
            unsafe {
                sys::Ptex_PtexWriter_close(
                    self.0,
                    std::ptr::addr_of_mut!(result),
                    std::ptr::addr_of_mut!(error_str),
                );
            }
            if !result {
                let default_error_message = "ptex: Writer::close() failed";
                let mut error_ptr: *const i8 = std::ptr::null_mut();
                unsafe {
                    let _error_msg = sys::std_string_c_str(
                        std::ptr::addr_of_mut!(error_str),
                        std::ptr::addr_of_mut!(error_ptr),
                    );
                    if !error_ptr.is_null() {
                        let cstr = CStr::from_ptr(error_ptr).to_str().or(Ok(default_error_message))?;
                        return Err(Error::String(cstr.to_string()));
                    }
                }
                return Err(Error::String(default_error_message.to_string()));
            }
            Ok(())
        }

        pub fn write_face_u16(
            &self,
            face_id: i32,
            face_info: &FaceInfo,
            data: &[u16],
            stride: i32,
        ) -> bool {
            if self.0.is_null() {
                return false;
            }
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
    */
}
