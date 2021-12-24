use crate::*;

use std::ffi::CString;

#[test]
fn open_and_write() {
    let filename = CString::new("out.ptx").unwrap();
    let alpha_channel = -1;
    let num_channels = 3;
    let num_faces = 9;
    let genmipmaps = false;

    let mut writer_ptr: *mut Ptex_PtexWriter_t = std::ptr::null_mut();
    let mut error_str = std_string_t::default();

    unsafe {
        Ptex_PtexWriter_open(
            std::ptr::addr_of_mut!(writer_ptr),
            filename.as_ptr(),
            Ptex_MeshType_mt_quad,
            Ptex_DataType_dt_uint8,
            num_channels,
            alpha_channel,
            num_faces,
            std::ptr::addr_of_mut!(error_str),
            genmipmaps,
        );
    }
}
