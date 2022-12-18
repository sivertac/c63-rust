use crate::c63;
use crate::encode_context;

use std::io::{self, Read};

fn read_file_to_buf(
    file: &mut std::fs::File,
    target_buffer: &mut [u8],
    bytes_to_read: usize,
) -> Result<usize, io::Error> {
    let mut len: usize = 0;
    while len < bytes_to_read {
        let res = file.read(&mut target_buffer[len..bytes_to_read])?;
        if res <= 0 {
            return Ok(len);
        }
        len += res;
    }
    return Ok(len);
}

pub fn read_yuv(
    file: &mut std::fs::File,
    ctx: &encode_context::EncodeContext,
) -> Result<c63::YUV, io::Error> {
    let mut len: usize = 0;
    let mut bytes_to_read: usize;
    let mut image = c63::YUV::new(
        (ctx.padw[c63::COLOR_COMPONENT_Y] * ctx.padh[c63::COLOR_COMPONENT_Y]) as usize,
        (ctx.padw[c63::COLOR_COMPONENT_U] * ctx.padh[c63::COLOR_COMPONENT_U]) as usize,
        (ctx.padw[c63::COLOR_COMPONENT_V] * ctx.padh[c63::COLOR_COMPONENT_V]) as usize,
    );

    /* Read Y. The size of Y is the same as the size of the image. The indices
    represents the color component (0 is Y, 1 is U, and 2 is V) */
    bytes_to_read = (ctx.width * ctx.height) as usize;
    len += read_file_to_buf(file, &mut image.y, bytes_to_read)?;

    /* Read U. Given 4:2:0 chroma sub-sampling, the size is 1/4 of Y
    because (height/2)*(width/2) = (height*width)/4. */
    bytes_to_read = ((ctx.width * ctx.height) / 4) as usize;
    len += read_file_to_buf(file, &mut image.u, bytes_to_read)?;

    /* Read V. Given 4:2:0 chroma sub-sampling, the size is 1/4 of Y. */
    bytes_to_read = ((ctx.width * ctx.height) / 4) as usize;
    len += read_file_to_buf(file, &mut image.v, bytes_to_read)?;

    if len <= 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "File empty, read returned 0",
        ));
    }

    if len != ((ctx.width * ctx.height) as f64 * 1.5) as usize {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "YUV size is wrong",
        ));
    }

    return Ok(image);
}
