
use clap::{arg, command, value_parser};

use c63_rust::c63;

#[derive(Debug)]
struct EncoderOptions {
    image_height: i32,
    image_width: i32,
    frames: Option<u64>, // number of frames to encode
    input_file:std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn main() {
    
    let matches = command!()
        .arg(arg!(<input_file> "Input file (.yuv)").required(true).value_parser(value_parser!(std::path::PathBuf)))
        .arg(arg!(--height <value> "Height of images to compress").required(true).value_parser(value_parser!(i32)))
        .arg(arg!(--width <value> "Width of images to compress").required(true).value_parser(value_parser!(i32)))
        .arg(arg!(-o --output <value> "Output file (.c63)").required(true).value_parser(value_parser!(std::path::PathBuf)))
        .arg(arg!(-f --frames <value> "Limit number of frames to encode").required(false).value_parser(value_parser!(u64)))
        .get_matches();
    
    let encoder_options = EncoderOptions{
        image_height: *matches.get_one::<i32>("height").expect("required"),
        image_width: *matches.get_one::<i32>("width").expect("required"),
        frames: matches.get_one::<u64>("frames").cloned(),
        input_file: matches.get_one::<std::path::PathBuf>("input_file").expect("required").clone(),
        output_file: matches.get_one::<std::path::PathBuf>("output").expect("required").clone(),
    };

    println!(
        "options: {:#?}",
        encoder_options
    );

    println!("{}", c63::PI)
}