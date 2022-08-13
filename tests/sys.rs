use std::ffi::CString;

#[test]
fn open_writer() {
    let filename = CString::new("out.ptx").unwrap();
    let alpha_channel = -1;
    let num_channels = 3;
    let num_faces = 9;
    let genmipmaps = false;

    let mut writer_ptr: *mut ptex_sys::Ptex_PtexWriter_t = std::ptr::null_mut();
    let mut error_str = ptex_sys::std_string_t::default();

    unsafe {
        ptex_sys::Ptex_PtexWriter_open(
            std::ptr::addr_of_mut!(writer_ptr),
            filename.as_ptr(),
            ptex_sys::Ptex_MeshType_mt_quad,
            ptex_sys::Ptex_DataType_dt_uint8,
            num_channels,
            alpha_channel,
            num_faces,
            std::ptr::addr_of_mut!(error_str),
            genmipmaps,
        );
    }
}
