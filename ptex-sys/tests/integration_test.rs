use cxx::let_cxx_string;
use ptex_sys::{
    ptexcache_create, ptexcache_get, ptextexture_get_border_mode_u, ptextexture_get_border_mode_v,
    ptextexture_get_edge_filter_mode, ptextexture_get_face_info, ptexwriter_close,
    ptexwriter_set_border_modes, ptexwriter_set_edge_filter_mode, ptexwriter_write_face,
    BorderMode, EdgeFilterMode, EdgeId, FaceInfo, PtexCache, PtexWriter,
};

#[test]
fn open_writer() {
    let filename = "out.ptx";
    let alpha_channel = -1;
    let num_channels = 3;
    let num_faces = 9;
    let genmipmaps = false;
    let meshtype = ptex_sys::MeshType::Quad;
    let datatype = ptex_sys::DataType::UInt8;

    let_cxx_string!(error_str = "");

    let writer = unsafe {
        ptex_sys::ptexwriter_open(
            filename,
            meshtype,
            datatype,
            num_channels,
            alpha_channel,
            num_faces,
            genmipmaps,
            error_str.as_mut().get_unchecked_mut(),
        )
    };
    assert!(!writer.is_null());
    assert!(error_str.is_empty());
}

#[test]
fn funky_values_mesh_type() {
    let filename = "out.ptx";
    let alpha_channel = -1;
    let num_channels = 3;
    let num_faces = 9;
    let genmipmaps = false;
    // The last variant in the enum.
    let mut meshtype = ptex_sys::MeshType::Quad;
    meshtype.repr += 1;
    let datatype = ptex_sys::DataType::Float32;

    let_cxx_string!(error_str = "");

    let writer = unsafe {
        ptex_sys::ptexwriter_open(
            filename,
            meshtype,
            datatype,
            num_channels,
            alpha_channel,
            num_faces,
            genmipmaps,
            error_str.as_mut().get_unchecked_mut(),
        )
    };
    assert!(writer.is_null());
    assert_eq!(
        error_str.to_str(),
        Ok("PtexWriter error: Invalid mesh type")
    );
}

#[test]
fn funky_values_data_type() {
    let filename = "out.ptx";
    let alpha_channel = -1;
    let num_channels = 3;
    let num_faces = 9;
    let genmipmaps = false;
    let meshtype = ptex_sys::MeshType::Quad;
    // The last variant in the enum.
    let mut datatype = ptex_sys::DataType::Float32;
    datatype.repr += 1;

    let_cxx_string!(error_str = "");

    let writer = unsafe {
        ptex_sys::ptexwriter_open(
            filename,
            meshtype,
            datatype,
            num_channels,
            alpha_channel,
            num_faces,
            genmipmaps,
            error_str.as_mut().get_unchecked_mut(),
        )
    };
    assert!(writer.is_null());
    assert_eq!(
        error_str.to_str(),
        Ok("PtexWriter error: Invalid data type")
    );
}

fn make_test_writer(filename: &str) -> *mut PtexWriter {
    let alpha_channel = -1;
    let num_channels = 3;
    let num_faces = 9;
    let genmipmaps = false;
    let meshtype = ptex_sys::MeshType::Quad;
    let datatype = ptex_sys::DataType::UInt8;

    let_cxx_string!(error_str = "");

    let writer = unsafe {
        ptex_sys::ptexwriter_open(
            filename,
            meshtype,
            datatype,
            num_channels,
            alpha_channel,
            num_faces,
            genmipmaps,
            error_str.as_mut().get_unchecked_mut(),
        )
    };
    assert!(!writer.is_null());
    writer
}

#[test]
fn funky_values_edge_id() {
    let writer = make_test_writer("funky_edge_ids.ptex");
    let mut face_info = FaceInfo::default();
    // Left currently being the last variant in `EdgeId`.
    let mut e1 = EdgeId::Left;
    let mut e2 = EdgeId::Left;
    let mut e3 = EdgeId::Left;
    let mut e4 = EdgeId::Left;
    e1.repr += 1;
    e2.repr += 2;
    e3.repr += 3;
    e4.repr += 4;
    let stride = 0;
    face_info.set_adjacent_edges(e1, e2, e3, e4);
    let data = [0_u8; 9];
    let wrote_face = unsafe { ptexwriter_write_face(writer, 0, &face_info, data.as_ptr(), stride) };
    assert!(wrote_face);
    unsafe {
        ptexwriter_close(writer);
    }
    let cache: *mut PtexCache = unsafe { ptexcache_create(1, 1024, true) };
    assert!(!cache.is_null());
    let_cxx_string!(error_str = "");
    let texture = unsafe {
        ptexcache_get(
            cache,
            "funky_edge_ids.ptex",
            error_str.as_mut().get_unchecked_mut(),
        )
    };
    assert!(!texture.is_null());
    let face_info = unsafe { ptextexture_get_face_info(texture, 0) };
    for i in 0..face_info.adjedges {
        let edge_id = face_info.adjacent_edge(i.into());
        assert!(
            edge_id == EdgeId::Bottom
                || edge_id == EdgeId::Right
                || edge_id == EdgeId::Top
                || edge_id == EdgeId::Left
        );
    }
}

#[test]
fn funky_values_border_modes() {
    let writer = make_test_writer("funky_border_modes.ptex");
    // The last variant in the enum.
    let mut border_mode = BorderMode::Periodic;
    border_mode.repr += 1;
    unsafe {
        ptexwriter_set_border_modes(writer, border_mode, border_mode);
    }
    let stride = 0;
    let mut face_info = FaceInfo::default();
    face_info.set_adjacent_edges(EdgeId::Bottom, EdgeId::Right, EdgeId::Top, EdgeId::Left);
    let data = [0_u8; 9];
    let wrote_face = unsafe { ptexwriter_write_face(writer, 0, &face_info, data.as_ptr(), stride) };
    assert!(wrote_face);
    unsafe {
        ptexwriter_close(writer);
    }
    let cache: *mut PtexCache = unsafe { ptexcache_create(1, 1024, true) };
    assert!(!cache.is_null());
    let_cxx_string!(error_str = "");
    let texture = unsafe {
        ptexcache_get(
            cache,
            "funky_border_modes.ptex",
            error_str.as_mut().get_unchecked_mut(),
        )
    };
    assert!(!texture.is_null());
    unsafe {
        assert!(ptextexture_get_border_mode_u(texture) != border_mode);
        assert!(ptextexture_get_border_mode_v(texture) != border_mode);
    }
}

#[test]
fn funky_values_edge_filter_mode() {
    let writer = make_test_writer("funky_edge_filter_mode.ptex");
    // The last variant in the enum.
    let mut edge_filter_mode = EdgeFilterMode::TangentVector;
    edge_filter_mode.repr += 1;
    unsafe {
        ptexwriter_set_edge_filter_mode(writer, edge_filter_mode);
    }
    let stride = 0;
    let mut face_info = FaceInfo::default();
    face_info.set_adjacent_edges(EdgeId::Bottom, EdgeId::Right, EdgeId::Top, EdgeId::Left);
    let data = [0_u8; 9];
    let wrote_face = unsafe { ptexwriter_write_face(writer, 0, &face_info, data.as_ptr(), stride) };
    assert!(wrote_face);
    unsafe {
        ptexwriter_close(writer);
    }
    let cache: *mut PtexCache = unsafe { ptexcache_create(1, 1024, true) };
    assert!(!cache.is_null());
    let_cxx_string!(error_str = "");
    let texture = unsafe {
        ptexcache_get(
            cache,
            "funky_edge_filter_mode.ptex",
            error_str.as_mut().get_unchecked_mut(),
        )
    };
    assert!(!texture.is_null());
    unsafe {
        assert!(ptextexture_get_edge_filter_mode(texture) != edge_filter_mode);
    }
}

#[test]
fn metadata_test() {
    use ptex_sys::MetaDataType;
    use std::ffi::{c_char, CString};

    let filename = "metadata.ptex";
    let value_str = (
        CString::new("test string metadata").unwrap(),
        CString::new("test_key1").unwrap(),
    );
    let value_i8 = ([i8::MAX; 3], CString::new("test_key2").unwrap());
    let value_i16 = ([i16::MAX; 3], CString::new("test_key3").unwrap());
    let value_i32 = ([i32::MAX; 6], CString::new("test_key4").unwrap());
    let value_float = ([f32::MAX; 5], CString::new("test_key5").unwrap());
    let value_double = ([f64::MAX; 3], CString::new("test_key6").unwrap());
    // Writing.
    {
        use ptex_sys::ptexwriter_write_meta_data;
        let writer = make_test_writer(filename);
        unsafe {
            ptexwriter_write_meta_data(
                writer,
                value_str.1.as_ptr() as *const c_char,
                MetaDataType::String,
                value_str.0.as_bytes_with_nul().as_ptr(),
                0,
            );
            ptexwriter_write_meta_data(
                writer,
                value_i8.1.as_ptr() as *const c_char,
                MetaDataType::Int8,
                value_i8.0.as_ptr() as *const u8,
                value_i8.0.len(),
            );
            ptexwriter_write_meta_data(
                writer,
                value_i16.1.as_ptr() as *const c_char,
                MetaDataType::Int16,
                value_i16.0.as_ptr() as *const u8,
                value_i16.0.len(),
            );
            ptexwriter_write_meta_data(
                writer,
                value_i32.1.as_ptr() as *const c_char,
                MetaDataType::Int32,
                value_i32.0.as_ptr() as *const u8,
                value_i32.0.len(),
            );
            ptexwriter_write_meta_data(
                writer,
                value_float.1.as_ptr() as *const c_char,
                MetaDataType::Float,
                value_float.0.as_ptr() as *const u8,
                value_float.0.len(),
            );
            ptexwriter_write_meta_data(
                writer,
                value_double.1.as_ptr() as *const c_char,
                MetaDataType::Double,
                value_double.0.as_ptr() as *const u8,
                value_double.0.len(),
            );
            ptexwriter_close(writer);
        }
    }
    // Reading
    {
        use ptex_sys::{ptexmetadata_get_value_for_key, ptextexture_get_meta_data};
        use std::ffi::CStr;
        let mut n = i32::MAX;
        let cache: *mut PtexCache = unsafe { ptexcache_create(1, 1024, true) };
        assert!(!cache.is_null());
        let_cxx_string!(error_str = "");
        let texture =
            unsafe { ptexcache_get(cache, filename, error_str.as_mut().get_unchecked_mut()) };
        assert!(!texture.is_null());
        let metadata = unsafe { ptextexture_get_meta_data(texture) };
        assert!(!metadata.is_null());
        unsafe {
            let mut x_str: *mut u8 = std::ptr::null_mut();
            ptexmetadata_get_value_for_key(
                metadata,
                value_str.1.as_ptr(),
                MetaDataType::String,
                &mut x_str as *mut *mut u8,
                &mut n,
            );
            assert_eq!(n, 0);
            assert!(!std::ptr::eq(x_str, std::ptr::null()));
            let expected_str = CString::new(value_str.0.clone()).unwrap();
            assert_eq!(CStr::from_ptr(x_str as *const i8), expected_str.as_c_str());
        }
        unsafe {
            let mut x_i8: *mut u8 = std::ptr::null_mut();
            ptexmetadata_get_value_for_key(
                metadata,
                value_i8.1.as_ptr(),
                MetaDataType::Int8,
                &mut x_i8 as *mut *mut u8,
                &mut n,
            );
            assert_eq!(n as usize, value_i8.0.len());
            assert!(!std::ptr::eq(x_i8, std::ptr::null()));
            let slice = std::slice::from_raw_parts(x_i8 as *const i8, n as usize);
            assert_eq!(slice, value_i8.0);
        }
        unsafe {
            let mut x_i16: *mut u8 = std::ptr::null_mut();
            ptexmetadata_get_value_for_key(
                metadata,
                value_i16.1.as_ptr(),
                MetaDataType::Int16,
                &mut x_i16 as *mut *mut u8,
                &mut n,
            );
            assert_eq!(n as usize, value_i16.0.len());
            assert!(!std::ptr::eq(x_i16, std::ptr::null()));
            let slice = std::slice::from_raw_parts(x_i16 as *const i16, n as usize);
            assert_eq!(slice, value_i16.0);
        }
        unsafe {
            let mut x_i32: *mut u8 = std::ptr::null_mut();
            ptexmetadata_get_value_for_key(
                metadata,
                value_i32.1.as_ptr(),
                MetaDataType::Int32,
                &mut x_i32 as *mut *mut u8,
                &mut n,
            );
            assert_eq!(n as usize, value_i32.0.len());
            assert!(!std::ptr::eq(x_i32, std::ptr::null()));
            let slice = std::slice::from_raw_parts(x_i32 as *const i32, n as usize);
            assert_eq!(slice, value_i32.0);
        }
        unsafe {
            let mut x_float: *mut u8 = std::ptr::null_mut();
            ptexmetadata_get_value_for_key(
                metadata,
                value_float.1.as_ptr(),
                MetaDataType::Float,
                &mut x_float as *mut *mut u8,
                &mut n,
            );
            assert_eq!(n as usize, value_float.0.len());
            assert!(!std::ptr::eq(x_float, std::ptr::null()));
            let slice = std::slice::from_raw_parts(x_float as *const f32, n as usize);
            assert_eq!(slice, value_float.0);
        }
        unsafe {
            let mut x_double: *mut u8 = std::ptr::null_mut();
            ptexmetadata_get_value_for_key(
                metadata,
                value_double.1.as_ptr(),
                MetaDataType::Double,
                &mut x_double as *mut *mut u8,
                &mut n,
            );
            assert_eq!(n as usize, value_double.0.len());
            assert!(!std::ptr::eq(x_double, std::ptr::null()));
            let slice = std::slice::from_raw_parts(x_double as *const f64, n as usize);
            assert_eq!(slice, value_double.0);
        }
    }
}
