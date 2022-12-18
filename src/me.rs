use crate::c63;
use crate::dsp;
use crate::encode_context;

fn me_block_8x8(
    macroblocks: &mut c63::MacroBlockContainer, // current frame
    mb_x: i32,
    mb_y: i32,
    original: &[u8],
    reference: &[u8],
    color_component: c63::ColorComponent,
    padw: &c63::PaddingContainer,
    padh: &c63::PaddingContainer,
    me_search_range: i32,
) {
    let mut mb =
        &mut macroblocks[color_component][(mb_y * padw[color_component] / 8 + mb_x) as usize];

    let mut range = me_search_range;

    /* Quarter resolution for chroma channels. */
    if color_component != c63::COLOR_COMPONENT_Y {
        range /= 2;
    }

    let mut left = mb_x * 8 - range;
    let mut top = mb_y * 8 - range;
    let mut right = mb_x * 8 + range;
    let mut bottom = mb_y * 8 + range;

    let w = padw[color_component];
    let h = padh[color_component];

    if (left < 0) {
        left = 0;
    }
    if (top < 0) {
        top = 0;
    }
    if (right > (w - 8)) {
        right = w - 8;
    }
    if (bottom > (h - 8)) {
        bottom = h - 8;
    }

    let mx = mb_x * 8;
    let my = mb_y * 8;

    let mut best_sad = std::i32::MAX;

    for y in top..bottom {
        for x in left..right {
            let sad = dsp::sad_block_8x8(&original, &reference, w);

            if sad < best_sad {
                mb.mv_x = (x - mx) as i8;
                mb.mv_y = (y - my) as i8;
                best_sad = sad;
            }
        }
    }

    /* Here, there should be a threshold on SAD that checks if the motion vector
    is cheaper than intraprediction. We always assume MV to be beneficial */

    /* printf("Using motion vector (%d, %d) with SAD %d\n", mb->mv_x, mb->mv_y,
    best_sad); */

    mb.use_mv = true;
}

pub fn c63_motion_estimate(
    current_frame: &mut c63::Frame,
    reference_frame: &c63::Frame,
    mb_rows: i32,
    mb_cols: i32,
    padw: &c63::PaddingContainer,
    padh: &c63::PaddingContainer,
    me_search_range: i32,
) {
    for mb_y in 0..mb_rows {
        for mb_x in 0..mb_cols {
            me_block_8x8(
                &mut current_frame.mbs,
                mb_x,
                mb_y,
                &current_frame.orig.y,
                &reference_frame.recons.y,
                c63::COLOR_COMPONENT_Y,
                padw,
                padh,
                me_search_range,
            );
        }
    }

    for mb_y in 0..mb_rows / 2 {
        for mb_x in 0..mb_cols / 2 {
            me_block_8x8(
                &mut current_frame.mbs,
                mb_x,
                mb_y,
                &current_frame.orig.u,
                &reference_frame.recons.u,
                c63::COLOR_COMPONENT_U,
                padw,
                padh,
                me_search_range,
            );
            me_block_8x8(
                &mut current_frame.mbs,
                mb_x,
                mb_y,
                &current_frame.orig.v,
                &reference_frame.recons.v,
                c63::COLOR_COMPONENT_V,
                padw,
                padh,
                me_search_range,
            );
        }
    }
}

fn mc_block_8x8(
    macroblocks: &c63::MacroBlockContainer, // current frame
    mb_x: i32,
    mb_y: i32,
    predicted: &mut [u8],
    reference: &[u8],
    color_component: c63::ColorComponent,
    padw: &c63::PaddingContainer,
    padh: &c63::PaddingContainer,
) {
    let mb = &macroblocks[color_component][(mb_y * padw[color_component] / 8 + mb_x) as usize];

    if !mb.use_mv {
        return;
    }

    let left = mb_x * 8;
    let top = mb_y * 8;
    let right = left + 8;
    let bottom = top + 8;

    let w = padw[color_component];

    /* Copy block from ref mandated by MV */
    for y in top..bottom {
        for x in left..right {
            predicted[(y * w + x) as usize] =
                reference[((y + mb.mv_y as i32) * w + (x + mb.mv_x as i32)) as usize];
        }
    }
}

pub fn c63_motion_compensate(
    current_frame: &mut c63::Frame,
    reference_frame: &c63::Frame,
    mb_rows: i32,
    mb_cols: i32,
    padw: &c63::PaddingContainer,
    padh: &c63::PaddingContainer,
) {
    /* Luma */
    for mb_y in 0..mb_rows {
        for mb_x in 0..mb_cols {
            mc_block_8x8(
                &current_frame.mbs,
                mb_x,
                mb_y,
                &mut current_frame.predicted.y,
                &reference_frame.recons.y,
                c63::COLOR_COMPONENT_Y,
                padw,
                padh,
            );
        }
    }

    /* Chroma */
    for mb_y in 0..mb_rows / 2 {
        for mb_x in 0..mb_cols / 2 {
            mc_block_8x8(
                &current_frame.mbs,
                mb_x,
                mb_y,
                &mut current_frame.predicted.u,
                &reference_frame.recons.u,
                c63::COLOR_COMPONENT_U,
                padw,
                padh,
            );
            mc_block_8x8(
                &current_frame.mbs,
                mb_x,
                mb_y,
                &mut current_frame.predicted.v,
                &reference_frame.recons.v,
                c63::COLOR_COMPONENT_V,
                padw,
                padh,
            );
        }
    }
}
