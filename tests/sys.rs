use cxx::let_cxx_string;
use ptex::sys;

use std::ffi::CString;

#[test]
fn open_writer() {
    let filename = CString::new("/out.ptx").unwrap();
    let alpha_channel = -1;
    let num_channels = 3;
    let num_faces = 9;
    let genmipmaps = false;
    let meshtype = sys::ffi::MeshType::Quad;
    let datatype = sys::ffi::DataType::UInt8;

    //let mut writer_ptr: *mut sys::ffi::PtexWriter = std::ptr::null_mut();
    let_cxx_string!(error_str = "");

    let writer = unsafe {
        sys::ffi::writer_open(
            filename.as_ptr(),
            meshtype,
            datatype,
            num_channels,
            alpha_channel,
            num_faces,
            genmipmaps,
            error_str.as_mut().get_unchecked_mut(),
        )
    };
    assert!(writer != std::ptr::null_mut());
    assert!(error_str.is_empty());
}
