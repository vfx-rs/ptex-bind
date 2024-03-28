use cxx::let_cxx_string;
use ptex_sys::{
    ptexcache_create, ptexcache_get, ptextexture_get_border_mode_u, ptextexture_get_border_mode_v,
    ptextexture_get_face_info, ptexwriter_close, ptexwriter_set_border_modes,
    ptexwriter_write_face, BorderMode, EdgeId, FaceInfo, PtexCache, PtexWriter,
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
