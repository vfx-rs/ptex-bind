#[cfg(test)]
use anyhow::Result;


#[test]
fn ptex_reader() -> Result<()> {
    let cache = ptex::reader::Cache::new(0, 0, false);

    let search_path = cache.search_path();
    assert_eq!("", search_path);
    Ok(())
}
