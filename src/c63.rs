
pub const ISQRT2: f64 = 0.70710678118654;
pub const PI: f64 = 3.14159265358979;
pub const ILOG2: f64 = 1.442695040888963; // 1/log(2);

pub enum ColorComponent {
    Y = 0,
    U = 1,
    V = 2
}
pub const COLOR_COMPONENTS: usize = 3; //std::mem::variant_count::<ColorComponent>();

pub const COMPONENTSIZE_YX: u8 = 2;
pub const COMPONENTSIZE_YY: u8 = 2;
pub const COMPONENTSIZE_UX: u8 = 1;
pub const COMPONENTSIZE_UY: u8 = 1;
pub const COMPONENTSIZE_VX: u8 = 1;
pub const COMPONENTSIZE_VY: u8 = 1;

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
    y: Box<[u8]>,
    u: Box<[u8]>,
    v: Box<[u8]>
}

pub struct DCT {
    y_dct: Box<[i16]>,
    u_dct: Box<[i16]>,
    v_dct: Box<[i16]>
}

pub struct MacroBlock {
    use_mv: i32,
    mv_x: i8,
    mv_y: i8
}

pub struct Frame {
    orig: Box<YUV>,                         // Original input image
    recons: Box<YUV>,                       // Reconstructed image
    predicted: Box<YUV>,                    // Predicted frame from intra-prediction

    residuals: Box<DCT>,                    // Difference between original image and predicted frame

    mbs: [MacroBlock; COLOR_COMPONENTS],    // macroblocks
    keyframe: i32
}

