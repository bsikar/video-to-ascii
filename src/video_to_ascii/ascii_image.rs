use crate::Flag;
use fontdue::{Font, FontSettings};
use image::{GenericImage, Pixel, Rgb, RgbImage};
use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

static FONT_FILE: &[u8] = include_bytes!("../../assets/fonts/consolas.ttf");
const ASCII_TABLE: [char; 70] = [
    '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b', 'd', 'p', 'q', 'w',
    'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 'z', 'c', 'v', 'u', 'n', 'x', 'r', 'j',
    'f', 't', '/', '\\', '|', '(', ')', '1', '{', '}', '[', ']', '?', '-', '_', '+', '~', '<', '>',
    'i', '!', 'l', 'I', ';', ':', ',', '"', '^', '`', '\'', '.', ' ',
];

pub struct AsciiImage(RgbImage);

impl AsciiImage {
    pub fn new(image: RgbImage) -> AsciiImage {
        AsciiImage(image)
    }

    pub fn output_to_stdout(&self, flags: Vec<Flag>) {
        let chars = self.get_ascii();

        let show_inverted = flags.contains(&Flag::Invert);
        let show_color = flags.contains(&Flag::Color);
        let show_ascii = flags.contains(&Flag::Ascii);

        for row in chars {
            for (ascii, color) in row {
                let mut color = Rgb(color);
                if show_inverted {
                    color.invert();
                }

                let mut stdout = StandardStream::stdout(ColorChoice::Always);
                let color = Some(termcolor::Color::Rgb(color[0], color[1], color[2]));
                if show_ascii && show_color {
                    // color and ascii
                    stdout.set_color(ColorSpec::new().set_bg(color)).unwrap();
                } else {
                    // only color
                    stdout
                        .set_color(ColorSpec::new().set_fg(color).set_bg(color))
                        .unwrap();
                }
                stdout.write_all(ascii.to_string().as_bytes()).unwrap();
            }
            println!();
        }
    }

    fn get_ascii(&self) -> Vec<Vec<(char, [u8; 3])>> {
        let mut characters = vec![vec![]];

        for row in self.0.rows() {
            let mut line = vec![];
            for pixel in row {
                let ascii = ASCII_TABLE[(pixel.0[0] as f64 / u8::MAX as f64
                    * (ASCII_TABLE.len() - 1) as f64)
                    .trunc() as usize];
                let rgb = [pixel.0[0], pixel.0[1], pixel.0[2]];

                line.push((ascii, rgb));
            }
            characters.push(line);
        }
        characters.remove(0);

        characters
    }

    pub fn output_to_file(&self, path: String, px: u32, flags: Vec<Flag>) {
        let font = Font::from_bytes(FONT_FILE, FontSettings::default()).unwrap();
        self.rasterize(font, px, flags).save(path).unwrap();
    }

    fn get_ascii_as_image(
        &self,
        c: char,
        rgb: Rgb<u8>,
        font: &Font,
        px: u32,
        flags: Vec<Flag>,
    ) -> RgbImage {
        let show_inverted = flags.contains(&Flag::Invert);
        let show_color = flags.contains(&Flag::Color);
        let show_ascii = flags.contains(&Flag::Ascii);

        // this will rasterize each character
        let mut rgb = rgb;
        if !show_color {
            rgb = Rgb([rgb.to_luma()[0]; 3]);
        }

        if show_inverted {
            rgb.invert();
        }

        let (metrics, bitmap) = font.rasterize(c, (px - 1) as f32);
        let mut img = RgbImage::from_pixel(px, px, rgb); // background of whole image

        let dx = (px as usize - metrics.width) >> 1;
        let dy = (px as usize - metrics.height) >> 1;
        let mut bitmap = bitmap.into_iter();

        // iter the entire bitmap
        for y in dy..metrics.height + dy {
            for x in dx..metrics.width + dx {
                let bitmap_value = bitmap.next().unwrap();
                let mut rgb = rgb;
                if bitmap_value != 0 && show_ascii {
                    rgb.invert()
                }

                img.put_pixel(
                    x as u32, y as u32, rgb, // changes background color of character
                )
            }
        }

        img
    }

    fn rasterize(&self, font: Font, px: u32, flags: Vec<Flag>) -> RgbImage {
        let mut img = RgbImage::new(self.0.width() * px, self.0.height() * px);
        let ascii = self.get_ascii();

        // iter the whole image
        for iy in 0..self.0.height() {
            for ix in 0..self.0.width() {
                let (c, pixel) = ascii[iy as usize][ix as usize];
                let mut sub_img = img.sub_image(ix * px, iy * px, px, px);
                // rasterize character by iterating through each pixel of a character
                let raster = self.get_ascii_as_image(c, Rgb(pixel), &font, px, flags.clone());

                // iter the sub image and place the rasterized character
                for sy in 0..px {
                    for sx in 0..px {
                        sub_img.put_pixel(sx, sy, *raster.get_pixel(sx, sy)); // puts cell of character into image
                    }
                }
            }
        }

        img
    }
}
