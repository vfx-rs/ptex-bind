use anyhow::Result;

//use assert_float_eq::{afe_is_f32_near, afe_near_error_msg, assert_f32_near};

#[test]
fn test_cache_search_path() -> Result<()> {
    let mut cache = ptex::Cache::new(0, 0, false);

    let search_path = cache.search_path();
    assert_eq!("", search_path);

    cache.set_search_path("/tmp:/tmp/ptex");
    let search_path = cache.search_path();
    assert_eq!("/tmp:/tmp/ptex", search_path);

    Ok(())
}

#[test]
fn test_cache_get() -> Result<()> {
    let filename = std::path::PathBuf::from("tests/fixtures/test.ptx");
    let mut cache = ptex::Cache::new(0, 0, false);
    let texture = cache.get(&filename)?;
    assert!(!texture.is_null());
    assert_eq!(texture.alpha_channel(), -1);
    assert_eq!(texture.num_channels(), 3);
    assert_eq!(texture.num_faces(), 9);
    assert!(!texture.has_edits());
    assert!(texture.has_mip_maps());
    assert_eq!(filename, texture.filename());
    assert_eq!(texture.mesh_type(), ptex::MeshType::Quad);
    assert_eq!(texture.data_type(), ptex::DataType::UInt16);
    assert_eq!(texture.edge_filter_mode(), ptex::EdgeFilterMode::None);

    Ok(())
}

#[test]
fn test_face_info() -> Result<()> {
    let filename = std::path::PathBuf::from("tests/fixtures/test.ptx");
    let mut cache = ptex::Cache::new(0, 0, false);
    let texture = cache.get(&filename)?;
    assert_eq!(texture.num_faces(), 9);

    let face_info = texture.face_info(0);
    assert!(!face_info.has_edits());
    assert!(!face_info.is_constant());
    assert!(!face_info.is_neighborhood_constant());
    assert!(!face_info.is_subface());
    assert_eq!(face_info.adjacent_edge(0), ptex::EdgeId::Top);

    let face_info = texture.face_info(0);
    assert_eq!(face_info.adjacent_face(0), 3);

    Ok(())
}

/*
#[test]
fn test_face_info_set_adjfaces() -> Result<()> {
    let filename = std::path::PathBuf::from("tests/fixtures/test.ptx");
    let mut cache = ptex::reader::Cache::new(0, 0, false);
    let texture = cache.get(&filename)?;
    assert_eq!(texture.num_faces(), 9);

    let mut face_info = texture.face_info(0);
    assert_eq!(face_info.adjacent_face(0), 0);
    assert_eq!(face_info.adjacent_face(1), 0);
    assert_eq!(face_info.adjacent_face(2), 0);
    assert_eq!(face_info.adjacent_face(3), 0);

    face_info.set_adjacent_faces(1, 2, 3, 4);
    assert_eq!(face_info.adjacent_face(0), 1);
    assert_eq!(face_info.adjacent_face(1), 2);
    assert_eq!(face_info.adjacent_face(2), 3);
    assert_eq!(face_info.adjacent_face(3), 4);

    let mut face_info = texture.face_info(1);
    assert_eq!(face_info.adjacent_edge(0), ptex::EdgeId::Bottom);
    assert_eq!(face_info.adjacent_edge(1), ptex::EdgeId::Bottom);
    assert_eq!(face_info.adjacent_edge(2), ptex::EdgeId::Bottom);
    assert_eq!(face_info.adjacent_edge(3), ptex::EdgeId::Bottom);

    face_info.set_adjacent_edges(
        ptex::EdgeId::Left,
        ptex::EdgeId::Right,
        ptex::EdgeId::Top,
        ptex::EdgeId::Bottom,
    );
    assert_eq!(face_info.adjacent_edge(0), ptex::EdgeId::Left);
    assert_eq!(face_info.adjacent_edge(1), ptex::EdgeId::Right);
    assert_eq!(face_info.adjacent_edge(2), ptex::EdgeId::Top);
    assert_eq!(face_info.adjacent_edge(3), ptex::EdgeId::Bottom);

    Ok(())
}

#[test]
fn test_texture_pixel() -> Result<()> {
    let filename = std::path::PathBuf::from("tests/fixtures/test.ptx");
    let mut cache = ptex::reader::Cache::new(0, 0, false);
    let texture = cache.get(&filename)?;

    assert!(texture.num_faces() > 1);
    let face_info = texture.face_info(0);
    let res = face_info.resolution();
    assert_eq!(res.u(), 1);
    assert_eq!(res.v(), 1);

    let pixel = texture.pixel_f32(0, 0, 1, 0, 1);
    assert_eq!(pixel, 0.0);

    let pixel = texture.pixel_f32(0, 0, 1, 1, 1);
    assert_f32_near!(pixel, 0.007873655);

    let pixel = texture.pixel_f32(0, 0, 1, 2, 1);
    assert_eq!(pixel, 1.0);

    let pixel = texture.pixel_f32(1, 1, 1, 1, 1);
    assert_f32_near!(pixel, 0.33333334);

    Ok(())
}

#[test]
fn test_faceinfo_set_resolution() -> Result<()> {
    let filename = std::path::PathBuf::from("tests/fixtures/test.ptx");
    let mut cache = ptex::reader::Cache::new(0, 0, false);
    let texture = cache.get(&filename)?;

    let mut face_info = texture.face_info(0);
    let res = face_info.resolution();
    assert_eq!(res.u(), 1);
    assert_eq!(res.v(), 1);

    let res = ptex::Res::from_uv_log2(3, 4);
    face_info.set_resolution(&res);

    let res = face_info.resolution();
    let base: i32 = 2;
    assert_eq!(res.u(), base.pow(3));
    assert_eq!(res.v(), base.pow(4));

    Ok(())
}
*/
