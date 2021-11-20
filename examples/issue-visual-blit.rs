use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let mut nc = unsafe { Nc::new()? };

    let width = 32;
    let height = 32;
    let buffer: Vec<u8> = vec![0xBB; (height * width) as usize * 3];

    let visual = NcVisual::from_rgb_packed(buffer.as_slice(), height, width * 3, width, 255)?;
    let vopt = NcVisualOptions::new(
        None,
        NcScale::NOSCALE,
        1,
        2,
        None,
        None,
        NcBlitter::PIXEL,
        0,
        0,
    );

    // let plane = visual.render(&mut nc, &vopt)?; // deprecated function

    // the new function renders to a root plane if the options don't specify a plane
    let plane = unsafe { visual.blit(&mut nc, Some(&vopt))? };
    plane.reparent(unsafe { nc.stdplane() })?;

    nc_render_sleep![nc, 2];

    plane.destroy()?;
    visual.destroy();
    unsafe { nc.stop()? };

    Ok(())
}
