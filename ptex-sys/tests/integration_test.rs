use cxx::let_cxx_string;

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
fn funky_values1() {
    let filename = "out.ptx";
    let alpha_channel = -1;
    let num_channels = 3;
    let num_faces = 9;
    let genmipmaps = false;
    // The last variant in the enum.
    let mut meshtype = ptex_sys::MeshType::Quad;
    meshtype.repr += 1;
    let mut datatype = ptex_sys::DataType::Float32;

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
fn funky_values2() {
    let filename = "out.ptx";
    let alpha_channel = -1;
    let num_channels = 3;
    let num_faces = 9;
    let genmipmaps = false;
    let mut meshtype = ptex_sys::MeshType::Quad;
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
