#[cfg(test)]

use anyhow::Result;

use std::cmp;


#[test]
fn ptex_writer() -> Result<()> {
    let face_res = [
        ptex_rs::Res::from_uv_log2(8, 7),
        ptex_rs::Res::from_value(0x0201),
        ptex_rs::Res::from_uv_log2(3, 1),
        ptex_rs::Res::from_value(0x0405),
        ptex_rs::Res::from_uv_log2(9, 8),
        ptex_rs::Res::from_value(0x0402),
        ptex_rs::Res::from_uv_log2(6, 2),
        ptex_rs::Res::from_value(0x0407),
        ptex_rs::Res::from_uv_log2(2, 1),
    ];
    let adjedges = [
        [2, 3, 0, 1],
        [2, 3, 0, 1],
        [2, 3, 0, 1],
        [2, 3, 0, 1],
        [2, 3, 0, 1],
        [2, 3, 0, 1],
        [2, 3, 0, 1],
        [2, 3, 0, 1],
        [2, 3, 0, 1],
    ];
    let adjfaces = [
        [3, 1, -1, -1],
        [4, 2, -1, 0],
        [5, -1, -1, 1],
        [6, 4, 0, -1],
        [7, 5, 1, 3],
        [8, -1, 2, 4],
        [-1, 7, 3, -1],
        [-1, 8, 4, 6],
        [-1, -1, 5, 7],
    ];

    let filename = std::path::PathBuf::from("ptex_writer.ptx");
    let num_faces: i32 = face_res.len() as i32;
    let mesh_type = ptex_rs::MeshType::Quad;
    let data_type = ptex_rs::DataType::Uint16;
    let num_channels = 3;
    let alpha_channel = -1;

    let ptex_writer = ptex_rs::writer::Writer::new(
        &filename,
        mesh_type,
        data_type,
        num_channels,
        alpha_channel,
        num_faces,
        false,  // generate_mipmaps
    )?;

    let one_value = ptex_rs::OneValue::from(data_type);
    let mut size = 0;
    for res in face_res.iter() {
        size = cmp::max(size, res.size());
    }

    Ok(())
}
