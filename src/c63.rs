//pub const ISQRT2: f64 = 0.70710678118654;
//pub const PI: f64 = 3.14159265358979;
pub const ILOG2: f64 = 1.442695040888963; // 1/log(2);
pub const ISQRT2: f64 = 1f64 / std::f64::consts::SQRT_2;
pub const PI: f64 = std::f64::consts::PI;
//pub const ILOG2: f64 = 1f64 / 2f64.log2(); // idk why this can't be compile time :|

pub type ColorComponent = usize;
pub const COLOR_COMPONENT_Y: ColorComponent = 0;
pub const COLOR_COMPONENT_U: ColorComponent = 1;
pub const COLOR_COMPONENT_V: ColorComponent = 2;

pub const COLOR_COMPONENTS: usize = 3; //std::mem::variant_count::<ColorComponent>();

pub const COMPONENT_SIZE_YX: u8 = 2;
pub const COMPONENT_SIZE_YY: u8 = 2;
pub const COMPONENT_SIZE_UX: u8 = 1;
pub const COMPONENT_SIZE_UY: u8 = 1;
pub const COMPONENT_SIZE_VX: u8 = 1;
pub const COMPONENT_SIZE_VY: u8 = 1;

/* The JPEG file format defines several parts and each part is defined by a
marker. A file always starts with 0xFF and is then followed by a magic number,
e.g., like 0xD8 in the SOI marker below. Some markers have a payload, and if
so, the size of the payload is written before the payload itself. */
pub enum JPEGMarker {
    DEF = 0xFF,
    SOI = 0xD8,
    DQT = 0xDB,
    SOF = 0xC0,
    DHT = 0xC4,
    SOS = 0xDA,
    EOI = 0xD9,
}

pub const HUFF_AC_ZERO: u8 = 16;
pub const HUFF_AC_SIZE: u8 = 11;

pub struct YUV {
    pub y: Vec<u8>,
    pub u: Vec<u8>,
    pub v: Vec<u8>,
}

impl YUV {
    pub fn new(y_size: usize, u_size: usize, v_size: usize) -> YUV {
        return YUV {
            y: vec![0; y_size],
            u: vec![0; u_size],
            v: vec![0; v_size],
        };
    }
}

pub struct DCT {
    pub y_dct: Vec<i16>,
    pub u_dct: Vec<i16>,
    pub v_dct: Vec<i16>,
}

impl DCT {
    pub fn new(y_size: usize, u_size: usize, v_size: usize) -> DCT {
        return DCT {
            y_dct: vec![0; y_size],
            u_dct: vec![0; u_size],
            v_dct: vec![0; v_size],
        };
    }
}

#[derive(Default, Clone)]
pub struct MacroBlock {
    pub use_mv: bool,
    pub mv_x: i8,
    pub mv_y: i8,
}

pub type MacroBlockContainer = [Vec<MacroBlock>; COLOR_COMPONENTS];

pub struct Frame {
    pub orig: YUV,      // Original input image
    pub recons: YUV,    // Reconstructed image
    pub predicted: YUV, // Predicted frame from intra-prediction

    pub residuals: DCT, // Difference between original image and predicted frame

    pub mbs: MacroBlockContainer, // macroblocks
    pub keyframe: bool,
}

pub type PaddingContainer = [i32; COLOR_COMPONENTS];

impl Frame {
    pub fn new(
        orig: YUV,
        padw: &PaddingContainer,
        padh: &PaddingContainer,
        mb_cols: i32,
        mb_rows: i32,
        keyframe: bool,
    ) -> Frame {
        return Frame {
            orig: orig,
            recons: YUV::new(
                (padw[COLOR_COMPONENT_Y] * padh[COLOR_COMPONENT_Y]) as usize,
                (padw[COLOR_COMPONENT_U] * padh[COLOR_COMPONENT_U]) as usize,
                (padw[COLOR_COMPONENT_V] * padh[COLOR_COMPONENT_V]) as usize,
            ),
            predicted: YUV::new(
                (padw[COLOR_COMPONENT_Y] * padh[COLOR_COMPONENT_Y]) as usize,
                (padw[COLOR_COMPONENT_U] * padh[COLOR_COMPONENT_U]) as usize,
                (padw[COLOR_COMPONENT_V] * padh[COLOR_COMPONENT_V]) as usize,
            ),
            residuals: DCT::new(
                (padw[COLOR_COMPONENT_Y] * padh[COLOR_COMPONENT_Y]) as usize,
                (padw[COLOR_COMPONENT_U] * padh[COLOR_COMPONENT_U]) as usize,
                (padw[COLOR_COMPONENT_V] * padh[COLOR_COMPONENT_V]) as usize,
            ),
            mbs: [
                vec![MacroBlock::default(); (mb_rows * mb_cols) as usize],
                vec![MacroBlock::default(); (mb_rows * mb_cols) as usize],
                vec![MacroBlock::default(); (mb_rows * mb_cols) as usize],
            ],
            keyframe: keyframe,
        };
    }
}
