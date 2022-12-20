use crate::c63;
use crate::tables;

pub fn sad_block_8x8(block1: &[u8], block2: &[u8], stride: i32) -> i32 {
    let mut result = 0;

    for v in 0..8 {
        for u in 0..8 {
            result += i32::abs(
                block2[(v * stride + u) as usize].wrapping_sub(block1[(v * stride + u) as usize])
                    as i32,
            );
        }
    }

    return result;
}

fn transpose_block(in_data: &[f32; 64], out_data: &mut [f32; 64]) {
    for i in 0..8 {
        for j in 0..8 {
            out_data[i * 8 + j] = in_data[j * 8 + i];
        }
    }
}

fn dct_ld(in_data: &[f32], out_data: &mut [f32]) {
    for i in 0..8 {
        let mut dct = 0.0f32;

        for j in 0..8 {
            dct += in_data[j] * tables::DCTLOOKUP[j][i];
        }

        out_data[i] = dct;
    }
}

fn idct_ld(in_data: &[f32], out_data: &mut [f32]) {
    for i in 0..8 {
        let mut idct = 0.0f32;

        for j in 0..8 {
            idct += in_data[j] * tables::DCTLOOKUP[i][j];
        }

        out_data[i] = idct;
    }
}

fn scale_block(in_data: &[f32; 64], out_data: &mut [f32; 64]) {
    for v in 0..8 {
        for u in 0..8 {
            let a1 = if u == 0 { c63::ISQRT2 as f32 } else { 1.0f32 };
            let a2 = if v == 0 { c63::ISQRT2 as f32 } else { 1.0f32 };

            out_data[v * 8 + u] = in_data[v * 8 + u] * a1 * a2;
        }
    }
}

fn quantize_block(in_data: &[f32; 64], out_data: &mut [f32; 64], quant_tbl: &[u8; 64]) {
    for zigzag in 0..64 {
        let u = tables::ZIGZAG_U[zigzag];
        let v = tables::ZIGZAG_V[zigzag];

        let dct = in_data[(v * 8 + u) as usize];

        /* Zig-zag and quantize */
        out_data[zigzag] = ((dct / 4.0f32) / quant_tbl[zigzag] as f32).round();
    }
}

fn dequantize_block(in_data: &[f32; 64], out_data: &mut [f32; 64], quant_tbl: &[u8; 64]) {
    for zigzag in 0..64 {
        let u = tables::ZIGZAG_U[zigzag];
        let v = tables::ZIGZAG_V[zigzag];

        let dct = in_data[(v * 8 + u) as usize];

        /* Zig-zag and de-quantize */
        out_data[zigzag] = (dct * quant_tbl[zigzag] as f32).round() / 4.0f32;
    }
}

pub fn dct_quant_block_8x8(in_data: &[i16; 64], out_data: &mut [i16], quant_tbl: &[u8; 64]) {
    let mut mb: [f32; 64] = [0.0f32; 64];
    let mut mb2: [f32; 64] = [0.0f32; 64];

    for i in 0..64 {
        mb2[i] = in_data[i] as f32;
    }

    /* Two 1D DCT operations with transpose */
    for v in 0..8 {
        let i = v * 8;
        dct_ld(&mb2[i..i + 8], &mut mb[i..i + 8]);
    }
    transpose_block(&mb, &mut mb2);
    for v in 0..8 {
        let i = v * 8;
        dct_ld(&mb2[i..i + 8], &mut mb[i..i + 8]);
    }
    transpose_block(&mb, &mut mb2);

    scale_block(&mb2, &mut mb);
    quantize_block(&mb, &mut mb2, quant_tbl);

    for i in 0..64 {
        out_data[i] = mb2[i] as i16;
    }
}

pub fn dequant_idct_block_8x8(in_data: &[i16], out_data: &mut [i16; 64], quant_tbl: &[u8; 64]) {
    let mut mb: [f32; 64] = [0.0f32; 64];
    let mut mb2: [f32; 64] = [0.0f32; 64];

    for i in 0..64 {
        mb[i] = in_data[i] as f32;
    }

    dequantize_block(&mb, &mut mb2, quant_tbl);
    scale_block(&mb2, &mut mb);

    /* Two 1D IDCT operations with transpose */
    for v in 0..8 {
        let i = v * 8;
        idct_ld(&mb[i..i + 8], &mut mb2[i..i + 8]);
    }
    transpose_block(&mb2, &mut mb);
    for v in 0..8 {
        let i = v * 8;
        idct_ld(&mb[i..i + 8], &mut mb2[i..i + 8]);
    }
    transpose_block(&mb2, &mut mb);

    for i in 0..64 {
        out_data[i] = mb[i] as i16;
    }
}
