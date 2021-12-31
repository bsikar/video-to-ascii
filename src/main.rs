#[macro_use]
extern crate clap;
use clap::Arg;

mod video_to_ascii;
use video_to_ascii::*;

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("input")
                .help("This is the video you are converting to ascii")
                .required(true)
                .index(1)
                .value_name("INPUT"),
        )
        .arg(
            Arg::with_name("output")
                .help("This is the output file, if not specified, stdout is used")
                .index(2)
                .value_name("OUTPUT"),
        )
        .arg(
            Arg::with_name("width")
                .help("The width of the output image")
                .short("w")
                .long("width")
                .value_name("WIDTH"),
        )
        .arg(
            Arg::with_name("height")
                .help("The height of the output image")
                .short("h")
                .long("height")
                .value_name("HEIGHT"),
        )
        .arg(
            Arg::with_name("filter")
                .help("The filter used to resize the image")
                .short("f")
                .long("filter")
                .value_name("FILTER")
                .possible_values(&["nearest", "triangle", "catmullrom", "gaussian", "lanczos3"])
                .default_value("nearest"),
        )
        .arg(
            Arg::with_name("pixel_size")
                .help("The pixel size used to resize the image")
                .short("px")
                .long("pixel_size")
                .value_name("PIXEL_SIZE")
                .default_value("16"),
        )
        .get_matches();

    let width = matches
        .value_of("width")
        .map(|x| x.parse::<u32>().expect("failed to parse to u32"));
    let height = matches
        .value_of("height")
        .map(|x| x.parse::<u32>().expect("failed to parse to u32"));
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output");
    let filter = matches.value_of("filter");
    let px = matches
        .value_of("pixel_size")
        .map(|x| x.parse::<u32>().expect("failed to parse to u32"))
        .unwrap();

    let mut ascii_video = AsciiVideo::new(input, output, width, height, filter, px);
    ascii_video.output();
}
