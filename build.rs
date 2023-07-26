use std::path::PathBuf;

use anyhow::Result;
use cxx_build::CFG;

fn main() -> Result<()> {
    let pkgconfig = pkg_config::probe_library("ptex")?;

    let include_paths = pkgconfig.include_paths.iter().map(PathBuf::as_path);
    CFG.exported_header_dirs.extend(include_paths);

    cxx_build::bridge("src/sys.rs").compile("ptex-sys");

    println!("cargo:rerun-if-changed=src/sys.rs");

    Ok(())
}
