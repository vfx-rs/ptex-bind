use anyhow::Result;

#[test]
fn ptex_res() -> Result<()> {
    let res = ptex::Res::from_uv(3, 4);
    let base = 2_i32;
    assert_eq!(res.u(), base.pow(3));
    assert_eq!(res.v(), base.pow(4));

    let res = res.clone_swapped();
    assert_eq!(res.u(), base.pow(4));
    assert_eq!(res.v(), base.pow(3));

    let size = res.size();
    assert_eq!(size, base.pow(3 + 4) as usize);
    assert_eq!(size, base.pow(3 + 4) as usize);

    let mut res = res.clone();
    res.swap_uv();
    assert_eq!(res.u(), base.pow(3));
    assert_eq!(res.v(), base.pow(4));

    let clamp_res = ptex::Res::from_uv(1, 2);
    res.clamp(clamp_res);
    assert_eq!(res.u(), base.pow(1));
    assert_eq!(res.v(), base.pow(2));

    let res = ptex::Res::from_uv(3, 4);
    let tile_res = ptex::Res::from_uv(2, 2);
    let tilesu = res.ntilesu(tile_res);
    assert_eq!(tilesu, 2);

    let tilesv = res.ntilesv(tile_res);
    assert_eq!(tilesv, 4);

    let tiles = res.ntiles(tile_res);
    assert_eq!(tiles, 8);

    Ok(())
}
