use crate::c63;
use crate::me;
use crate::tables;

pub struct EncodeContext {
    pub width: i32,
    pub height: i32,

    pub ypw: i32,
    pub yph: i32,
    pub upw: i32,
    pub uph: i32,
    pub vpw: i32,
    pub vph: i32,

    pub padw: c63::PaddingContainer,
    pub padh: c63::PaddingContainer,

    pub mb_cols: i32,
    pub mb_rows: i32,

    pub qp: u8, // Quality parameter

    pub me_search_range: i32,

    pub quanttbl: [[u8; 64]; c63::COLOR_COMPONENTS],

    pub reference_frame: Option<Box<c63::Frame>>,
    pub current_frame: Option<Box<c63::Frame>>,

    pub framenum: i32,

    pub keyframe_interval: i32,
    pub frames_since_keyframe: i32,
}

impl EncodeContext {
    pub fn new(width: i32, height: i32) -> Result<EncodeContext, &'static str> {
        if width <= 0 {
            return Err("Invalid width, width must be positive nonzero integer");
        }
        if height <= 0 {
            return Err("Invalid height, height must be positive nonzero integer");
        }

        // compute padding (or something like that idk...)
        let ypw: i32 = (width as f32 / 16.0f32).ceil() as i32 * 16;
        let yph: i32 = (height as f32 / 16.0f32).ceil() as i32 * 16;
        let upw: i32 = (((width * c63::COMPONENT_SIZE_UX as i32) as f32)
            / (c63::COMPONENT_SIZE_YX as f32 * 8.0f32))
            .ceil() as i32
            * 8;
        let uph: i32 = (((height * c63::COMPONENT_SIZE_UY as i32) as f32)
            / (c63::COMPONENT_SIZE_YY as f32 * 8.0f32))
            .ceil() as i32
            * 8;
        let vpw: i32 = (((width * c63::COMPONENT_SIZE_VX as i32) as f32)
            / (c63::COMPONENT_SIZE_YX as f32 * 8.0f32))
            .ceil() as i32
            * 8;
        let vph: i32 = (((height * c63::COMPONENT_SIZE_VY as i32) as f32)
            / (c63::COMPONENT_SIZE_YY as f32 * 8.0f32))
            .ceil() as i32
            * 8;

        // Quality parameters
        let qp: u8 = 25;
        let me_search_range: i32 = 16;
        let keyframe_interval: i32 = 100;

        let mut quanttbl: [[u8; 64]; c63::COLOR_COMPONENTS] = [[0; 64]; c63::COLOR_COMPONENTS];
        for i in 0..64 {
            quanttbl[c63::COLOR_COMPONENT_Y][i] =
                tables::YQUANTTBL_DEF[i] / (qp as f64 / 10f64) as u8;
            quanttbl[c63::COLOR_COMPONENT_U][i] =
                tables::UVQUANTTBL_DEF[i] / (qp as f64 / 10f64) as u8;
            quanttbl[c63::COLOR_COMPONENT_V][i] =
                tables::UVQUANTTBL_DEF[i] / (qp as f64 / 10f64) as u8;
        }

        return Ok(EncodeContext {
            width: width,
            height: height,
            ypw: ypw,
            yph: yph,
            upw: upw,
            uph: uph,
            vpw: vpw,
            vph: vph,
            padw: [ypw, upw, vpw],
            padh: [yph, uph, vph],
            mb_cols: ypw / 8,
            mb_rows: yph / 8,
            // Quality parameters
            qp: qp,
            me_search_range: me_search_range,

            quanttbl: quanttbl,

            reference_frame: None,
            current_frame: None,

            framenum: 0,

            keyframe_interval: keyframe_interval,
            frames_since_keyframe: 0,
        });
    }
}

pub fn encode_image(ctx: &mut EncodeContext, image: c63::YUV) {
    ctx.reference_frame = ctx.current_frame.take();

    // check if this is a keyframe
    let mut keyframe: bool = false;
    if ctx.framenum == 0 || ctx.frames_since_keyframe == ctx.keyframe_interval {
        keyframe = true;
        ctx.frames_since_keyframe = 0;
    }
    ctx.current_frame = Some(Box::new(c63::Frame::new(
        image,
        &ctx.padw,
        &ctx.padh,
        ctx.mb_cols,
        ctx.mb_rows,
        keyframe,
    )));
    let current_frame = ctx.current_frame.as_mut().unwrap();

    if !current_frame.keyframe {
        let reference_frame = ctx.reference_frame.as_ref().unwrap();

        /* Motion Estimation */
        me::c63_motion_estimate(
            current_frame,
            reference_frame,
            ctx.mb_rows,
            ctx.mb_cols,
            &ctx.padw,
            &ctx.padh,
            ctx.me_search_range,
        );

        /* Motion Compensation */
        //c63_motion_compensate(cm);
    }

    ctx.framenum += 1;
    ctx.frames_since_keyframe += 1;
}
