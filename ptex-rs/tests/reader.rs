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
