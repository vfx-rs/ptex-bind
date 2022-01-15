#[cfg(test)]

use anyhow::Result;

use ptex_rs::writer;

#[test]
fn ptex_writer() -> Result<()> {
    let filename = std::path::PathBuf::from("ptex_writer.ptx");
    let ptex_writer = writer::Writer::new(&filename);

    Ok(())
}
