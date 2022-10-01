
use clap::{arg, command, value_parser};

#[derive(Debug)]
struct DecoderOptions {    
    input_file:std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn main() {
    
    let matches = command!()
        .arg(arg!(<input_file> "Input file (.c63)").required(true).value_parser(value_parser!(std::path::PathBuf)))
        .arg(arg!(<output_file> "Output file (.yuv)").required(true).value_parser(value_parser!(std::path::PathBuf)))
        .get_matches();
    
    let encoder_options = DecoderOptions{
        input_file: matches.get_one::<std::path::PathBuf>("input_file").expect("required").clone(),
        output_file: matches.get_one::<std::path::PathBuf>("output_file").expect("required").clone(),
    };

    println!(
        "options: {:#?}",
        encoder_options
    );

}