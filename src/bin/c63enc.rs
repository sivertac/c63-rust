use clap::{arg, command, value_parser};

use c63_rust::{c63, encode_context, yuv};

#[derive(Debug)]
struct EncoderOptions {
    image_height: u32,
    image_width: u32,
    frames: Option<u64>, // number of frames to encode
    input_file: std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn main() {
    let matches = command!()
        .arg(
            arg!(<input_file> "Input file (.yuv)")
                .required(true)
                .value_parser(value_parser!(std::path::PathBuf)),
        )
        .arg(
            arg!(--height <value> "Height of images to compress")
                .required(true)
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--width <value> "Width of images to compress")
                .required(true)
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(-o --output <value> "Output file (.c63)")
                .required(true)
                .value_parser(value_parser!(std::path::PathBuf)),
        )
        .arg(
            arg!(-f --frames <value> "Limit number of frames to encode")
                .required(false)
                .value_parser(value_parser!(u64)),
        )
        .get_matches();

    let encoder_options = EncoderOptions {
        image_height: *matches.get_one::<u32>("height").expect("required"),
        image_width: *matches.get_one::<u32>("width").expect("required"),
        frames: matches.get_one::<u64>("frames").cloned(),
        input_file: matches
            .get_one::<std::path::PathBuf>("input_file")
            .expect("required")
            .clone(),
        output_file: matches
            .get_one::<std::path::PathBuf>("output")
            .expect("required")
            .clone(),
    };

    println!("options: {:#?}", encoder_options);

    let mut ctx = encode_context::EncodeContext::new(
        encoder_options.image_width as i32,
        encoder_options.image_height as i32,
    )
    .unwrap();

    let mut output_file = std::fs::File::create(encoder_options.output_file).unwrap();
    let mut input_file = std::fs::File::open(encoder_options.input_file).unwrap();

    let mut num_frames = 0;

    loop {
        println!("{}", num_frames);

        // read frame
        let image = match yuv::read_yuv(&mut input_file, &ctx) {
            // if file empty then break
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => panic!("{}", e),
            Ok(i) => i,
        };

        // encode image
        encode_context::encode_image(&mut ctx, image);

        yuv::dump_image(
            &ctx.current_frame.as_ref().unwrap().predicted,
            ctx.width,
            ctx.height,
            &mut output_file,
        )
        .unwrap();

        num_frames += 1;
        // if frame limit is set and num_frames is over limit then break
        if encoder_options.frames.is_some() && num_frames >= encoder_options.frames.unwrap() {
            break;
        }
    }
}
