use anyhow::Result;

// use std::cmp;
use std::fs;

#[test]
fn ptex_writer() -> Result<()> {
    let face_res = [
        ptex::Res::from_uv(8, 7),
        ptex::Res::from_value(0x0201),
        ptex::Res::from_uv(3, 1),
        ptex::Res::from_value(0x0405),
        ptex::Res::from_uv(9, 8),
        ptex::Res::from_value(0x0402),
        ptex::Res::from_uv(6, 2),
        ptex::Res::from_value(0x0407),
        ptex::Res::from_uv(2, 1),
    ];
    let _adjacent_edges = [
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
    let _adjacent_faces = [
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

    let filename = std::path::PathBuf::from("tests/tmp/ptex_writer.ptx");
    let _num_faces: i32 = face_res.len() as i32;
    let _mesh_type = ptex::MeshType::Quad;
    let _data_type = ptex::DataType::UInt16;
    let _num_channels = 3;
    let _alpha_channel = -1;

    if filename.exists() {
        fs::remove_file(&filename)?;
    }

    /*
    let ptex_writer = ptex::writer::Writer::new(
        &filename,
        mesh_type,
        data_type,
        num_channels,
        alpha_channel,
        num_faces,
        false, // generate_mipmaps
    )?;

    // Calculate the size for the u16 buffer used by write_face_u16()

    let mut size = 0;
    for res in face_res.iter() {
        size = cmp::max(size, res.size());
    }
    size *= num_channels as usize;

    let stride = 0;
    let one_value = ptex::OneValue::get(data_type);

    let mut buf: Vec<u16> = Vec::new();
    buf.resize(size, 0);

    for i in 0..num_faces as usize {
        buf.fill(0);

        let ures = face_res[i].u();
        let vres = face_res[i].v();

        for v in 0..vres {
            for u in 0..ures {
                let color = ((u ^ v) & 1) as f32;
                let idx = (((v * ures) + u) * num_channels) as usize;

                buf[idx] = ((u as f32) / (((ures - 1) as f32) * one_value)) as u16;
                buf[idx + 1] = ((v as f32) / (((vres - 1) as f32) * one_value)) as u16;
                buf[idx + 2] = (color * one_value) as u16;
            }
        }

        let face_info = ptex::FaceInfo::from_res_and_adjacency(
            &face_res[i],
            &adjacent_faces[i],
            &adjacent_edges[i],
            false,
        );

        assert!(ptex_writer.write_face_u16(i as i32, &face_info, &buf, stride));
    }

    assert_eq!(ptex_writer.close(), Ok(()));
    assert!(filename.exists());
    fs::remove_file(&filename)?;
    */

    Ok(())
}
