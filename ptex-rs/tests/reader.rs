#[cfg(test)]
use anyhow::Result;

#[test]
fn test_cache_search_path() -> Result<()> {
    let mut cache = ptex::reader::Cache::new(0, 0, false);

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
    let mut cache = ptex::reader::Cache::new(0, 0, false);
    let texture = cache.get(&filename)?;

    assert!(!texture.is_null());
    assert_eq!(texture.alpha_channel(), -1);
    assert_eq!(texture.num_channels(), 3);
    assert_eq!(texture.num_faces(), 9);
    assert!(!texture.has_edits());
    assert!(texture.has_mip_maps());
    assert_eq!(filename, texture.path());
    assert_eq!(texture.mesh_type(), ptex::reader::MeshType::Quad);
    assert_eq!(texture.data_type(), ptex::reader::DataType::Uint16);
    assert_eq!(
        texture.edge_filter_mode(),
        ptex::reader::EdgeFilterMode::None
    );

    Ok(())
}
