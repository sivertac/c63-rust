
use crate::c63;
use crate::tables;

pub struct EncodeContext {
    width: i32,
    height: i32,
    
    ypw: i32, 
    yph: i32, 
    upw: i32, 
    uph: i32, 
    vpw: i32, 
    vph: i32,

    padw: [i32; c63::COLOR_COMPONENTS],
    padh: [i32; c63::COLOR_COMPONENTS],

    mb_cols: i32,
    mb_rows: i32,

    qp: u8, // Quality parameter

    me_search_range: i32,
    keyframe_interval: i32,

    quanttbl: [[u8; 64]; c63::COLOR_COMPONENTS],
}

impl EncodeContext {
    fn new(width: i32, height: i32) -> Result<EncodeContext, &'static str> {
        if width <= 0 {
            return Err("Invalid width, width must be positive nonzero integer");
        }
        if height <= 0 {
            return Err("Invalid height, height must be positive nonzero integer");
        }

        // compute padding (or something like that idk...)
        let ypw: i32 = (width as f32 / 16.0f32).ceil() as i32 * 16;
        let yph: i32 = (height as f32 / 16.0f32).ceil() as i32 * 16;
        let upw: i32 = (((width * c63::COMPONENT_SIZE_UX as i32) as f32) / (c63::COMPONENT_SIZE_YX as f32 * 8.0f32)).ceil() as i32 * 8;
        let uph: i32 = (((height * c63::COMPONENT_SIZE_UY as i32) as f32) / (c63::COMPONENT_SIZE_YY as f32 * 8.0f32)).ceil() as i32 * 8;
        let vpw: i32 = (((width * c63::COMPONENT_SIZE_VX as i32) as f32) / (c63::COMPONENT_SIZE_YX as f32 * 8.0f32)).ceil() as i32 * 8;
        let vph: i32 = (((height * c63::COMPONENT_SIZE_VY as i32) as f32) / (c63::COMPONENT_SIZE_YY as f32 * 8.0f32)).ceil() as i32 * 8;

        // Quality parameters
        let qp: u8 = 25; 
        let me_search_range: i32 = 16; 
        let keyframe_interval: i32 = 100;

        let mut quanttbl: [[u8; 64]; c63::COLOR_COMPONENTS] = [[0; 64]; c63::COLOR_COMPONENTS];
        for i in 0..64 {
            quanttbl[c63::COLOR_COMPONENT_Y][i] = tables::YQUANTTBL_DEF[i] / (qp as f64 / 10f64) as u8;
            quanttbl[c63::COLOR_COMPONENT_U][i] = tables::UVQUANTTBL_DEF[i] / (qp as f64 / 10f64) as u8;
            quanttbl[c63::COLOR_COMPONENT_V][i] = tables::UVQUANTTBL_DEF[i] / (qp as f64 / 10f64) as u8;
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
            keyframe_interval: keyframe_interval, 
            
            quanttbl: quanttbl
        })
    }
}



