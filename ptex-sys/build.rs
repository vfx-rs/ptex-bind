use std::path::PathBuf;

use anyhow::Result;
use cxx_build::CFG;

fn main() -> Result<()> {
    // Skip linking on docs.rs: https://docs.rs/about/builds#detecting-docsrs
    let building_docs = std::env::var("DOCS_RS").is_ok();
    if building_docs {
        println!("cargo:rustc-cfg=docsrs");
        return Ok(());
    }

    let pkgconfig = pkg_config::probe_library("ptex")?;

    let mut include_paths: Vec<_> = pkgconfig
        .include_paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut bind_path = std::path::PathBuf::from(manifest_dir);
    bind_path.push("src");
    include_paths.push(bind_path.as_path());
    CFG.exported_header_dirs.extend(include_paths);

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-fpermissive")
        .compile("ptex-sys");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ptex-sys.h");

    Ok(())
}
