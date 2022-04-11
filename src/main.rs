use clap::Arg;

mod video_to_ascii;
use video_to_ascii::*;

fn main() {
    let matches = clap::command!()
        .arg(
            Arg::new("input")
                .help("This is the video you are converting to ascii")
                .required(true)
                .short('i')
                .long("input")
                .value_name("INPUT"),
        )
        .arg(
            Arg::new("output")
                .help("This is the output file, if not specified, stdout is used")
                .value_name("OUTPUT")
                .short('o')
                .long("output")
                .long("output"),
        )
        .arg(
            Arg::new("width")
                .help("The width of the output image\nIf no width is given then it will scale the width based off the specified height, but if no height is specified then the input video's dimensions will be used")
                .short('w')
                .long("width")
                .value_name("WIDTH"),
        )
        .arg(
            Arg::new("height")
                .help("The height of the output image\nIf no height is given then it will scale the height based off the specified width, but if no width is specified then the input video's dimensions will be used")
                .short('h')
                .long("height")
                .value_name("HEIGHT"),
        )
        .arg(
            Arg::new("filter")
                .help("The filter used to resize the image")
                .short('f')
                .long("filter")
                .value_name("FILTER")
                .possible_values(&["nearest", "triangle", "catmullrom", "gaussian", "lanczos3"])
                .default_value("nearest"),
        )
        .arg(
            Arg::new("pixel_size")
                .help("The pixel size used to resize the image")
                .short('p')
                .long("pixel_size")
                .value_name("PIXEL_SIZE")
                .default_value("16"),
        )
        .arg(
            Arg::new("show_ascii")
                .help("Should the output show ascii or not")
                .short('a')
                .long("show_ascii")
                .possible_values(&["true", "false"])
                .value_name("SHOW_ASCII")
                .default_value("true"),
        )
        .arg(
            Arg::new("show_color")
                .help("Should the output show color or not")
                .short('c')
                .long("show_color")
                .possible_values(&["true", "false"])
                .value_name("SHOW_COLOR")
                .default_value("true"),
        )
        .arg(
            Arg::new("show_inverted")
                .help("Should the output's color be inverted or not")
                .short('x')
                .long("show_inverted")
                .possible_values(&["true", "false"])
                .value_name("SHOW_INVERTED")
                .default_value("false"),
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
    let filter = matches.value_of("filter").unwrap();
    let px = matches
        .value_of("pixel_size")
        .map(|x| x.parse::<u32>().expect("failed to parse to u32"))
        .unwrap();
    let ascii = matches
        .value_of("show_ascii")
        .map(|x| x.parse::<bool>().expect("failed to parse to bool"))
        .unwrap();
    let color = matches
        .value_of("show_color")
        .map(|x| x.parse::<bool>().expect("failed to parse to bool"))
        .unwrap();
    let inverted = matches
        .value_of("show_inverted")
        .map(|x| x.parse::<bool>().expect("failed to parese to bool"))
        .unwrap();

    let mut flags = vec![];
    if ascii {
        flags.push(Flag::Ascii);
    }
    if color {
        flags.push(Flag::Color);
    }
    if inverted {
        flags.push(Flag::Invert);
    }

    let mut ascii_video = AsciiVideo::new((input, output), (width, height, px), filter, flags);
    ascii_video.output();
}
