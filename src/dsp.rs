pub fn sad_block_8x8(block1: &[u8], block2: &[u8], stride: i32) -> i32 {
    let mut result = 0;

    for v in 0..8 {
        for u in 0..8 {
            result += i32::abs(
                (block2[(v * stride + u) as usize] - block1[(v * stride + u) as usize]) as i32,
            );
        }
    }

    return result;
}
