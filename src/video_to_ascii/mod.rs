use std::{io::Write, process::Command};

use image::{
    imageops::{resize, FilterType},
    GenericImageView,
};
use terminal_size::terminal_size;

mod ascii_image;
use ascii_image::*;

mod videostream;
use videostream::*;

pub struct AsciiVideo<'a> {
    video: VideoStream,
    output: Option<&'a str>,
    width: Option<u32>,
    height: Option<u32>,
    filter: FilterType,
    px: u32,
    total_frames: usize,
    show_ascii: bool,
}

impl<'a> AsciiVideo<'a> {
    pub fn new(
        input: &'a str,
        output: Option<&'a str>,
        width: Option<u32>,
        height: Option<u32>,
        filter: Option<&str>,
        px: u32,
        show_ascii: bool,
    ) -> Self {
        let mut video = VideoStream::new(input).unwrap();
        let total_frames = video.iter().fold(0, |acc, _| acc + 1);
        let video = VideoStream::new(input).unwrap();

        let filter = match filter {
            Some(filter) => match filter.to_lowercase().as_str() {
                "Triangle" => FilterType::Triangle,
                "CatmullRom" => FilterType::CatmullRom,
                "Lanczos3" => FilterType::Lanczos3,
                _ => FilterType::Nearest,
            },
            None => FilterType::Nearest,
        };

        Self {
            video,
            output,
            width,
            height,
            filter,
            px,
            total_frames,
            show_ascii,
        }
    }

    pub fn output(&mut self) {
        if self.output.is_some() {
            self.file_output();
        } else {
            self.stdout_output();
        }
    }

    fn file_output(&mut self) {
        let dir = ".video_to_ascii_tmp";
        let prefix = self.output.unwrap().split('.').next().unwrap();
        if !std::path::Path::new(dir).exists() {
            std::fs::create_dir(dir).unwrap();
        }

        for (cnt, frame) in self.video.iter().enumerate() {
            let img = frame.as_rgb().unwrap();
            let img = match Self::width_and_height(self.width, self.height, self.output, &img) {
                Some((width, height)) => resize(&img, width, height, self.filter),
                None => img,
            };

            let path = format!("{}/{}_{}.png", dir, prefix, cnt);

            print!("{} / {}\r", cnt, self.total_frames);
            std::io::stdout().flush().unwrap();

            if self.show_ascii {
                AsciiImage::new(img).output_to_file(path, self.px, self.show_ascii);
            } else {
                img.save(path).unwrap();
            }
        }
        // save all files to mp4 and remove each file one added to the mp4
        // TODO make this work without Command
        Command::new("ffmpeg")
            .args(&[
                "-i",
                &format!("{}/{}_%d.png", dir, prefix),
                &format!("{}", self.output.unwrap()),
            ])
            .output()
            .expect("failed to execute process");

        std::fs::remove_dir_all(dir).unwrap();
    }

    fn stdout_output(&mut self) {
        for frame in self.video.iter() {
            let img = frame.as_rgb().unwrap();
            let width_and_height =
                Self::width_and_height(self.width, self.height, self.output, &img);
            let img = match width_and_height {
                Some((width, height)) => resize(&img, width, height, self.filter),
                None => img,
            };

            let ascii_image = AsciiImage::new(img);
            ascii_image.output_to_stdout(self.show_ascii);
            print!("\x1B[1;1H"); // move curser
        }
    }

    fn width_and_height<I: GenericImageView>(
        width: Option<u32>,
        height: Option<u32>,
        output: Option<&'a str>,
        image: &I,
    ) -> Option<(u32, u32)> {
        if width.is_some() && height.is_some() {
            Some((width.unwrap(), height.unwrap()))
        } else if width.is_none() && height.is_none() && output.is_none() {
            let (width, height) = terminal_size().unwrap();
            Some((width.0 as u32, height.0 as u32))
        } else if width.is_none() && height.is_none() && output.is_some() {
            None
        } else if width.is_some() {
            let height = width.unwrap() as f64 / image.width() as f64 * image.height() as f64;
            Some((width.unwrap(), height as u32))
        } else {
            let width = height.unwrap() as f64 / image.height() as f64 * image.width() as f64;
            Some((width as u32, height.unwrap()))
        }
    }
}
