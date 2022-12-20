use crate::dsp;

fn dct_quantize_row(
    in_data: &[u8],
    prediction: &[u8],
    w: i32,
    _h: i32,
    out_data: &mut [i16],
    quantization: &[u8; 64],
) {
    let mut block = [0i16; 64];

    /* Perform the DCT and quantization */
    for x in (0..w as usize).step_by(8) {
        for i in 0..8usize {
            for j in 0..8usize {
                let data_index = i * w as usize + j + x;
                block[(i * 8 + j) as usize] =
                    in_data[data_index] as i16 - prediction[data_index] as i16;
            }
        }

        /* Store MBs linear in memory, i.e. the 64 coefficients are stored
        continous. This allows us to ignore stride in DCT/iDCT and other
        functions. */
        dsp::dct_quant_block_8x8(&block, &mut out_data[x..x + 64 as usize], quantization);
    }
}

pub fn dct_quantize(
    in_data: &[u8],
    prediction: &[u8],
    width: i32,
    height: i32,
    out_data: &mut [i16],
    quantization: &[u8; 64],
) {
    for y in (0..height as usize).step_by(8) {
        let data_index = y * width as usize;
        dct_quantize_row(
            &in_data[data_index..],
            &prediction[data_index..],
            width,
            height,
            &mut out_data[data_index..],
            quantization,
        );
    }
}
