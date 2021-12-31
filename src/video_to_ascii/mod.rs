use std::{sync::atomic::{AtomicUsize, Ordering}, io::Write};

use image::{
    imageops::{resize, FilterType},
    GenericImageView,
};
use rayon::prelude::*;
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
}

impl<'a> AsciiVideo<'a> {
    pub fn new(
        input: &'a str,
        output: Option<&'a str>,
        width: Option<u32>,
        height: Option<u32>,
        filter: Option<&str>,
        px: u32,
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
        let i = AtomicUsize::new(0);
        self.video.iter().par_bridge().for_each(|frame| {
            let cnt = i.fetch_add(1, Ordering::Relaxed);
            let img = frame.as_rgb().unwrap();
            let width_and_height =
                Self::width_and_height(self.width, self.height, self.output, &img);
            let img = match width_and_height {
                Some((width, height)) => resize(&img, width, height, self.filter),
                None => img,
            };

            let ascii_image = AsciiImage::new(img);

            let prefix = self.output.unwrap().split('.').next().unwrap();
            let extension = self.output.unwrap().split('.').last().unwrap();
            // make dir
            let dir = ".video_to_ascii_tmp";
            if !std::path::Path::new(dir).exists() {
                std::fs::create_dir(dir).unwrap();
            }
            let path = format!("{}/{}_{}.{}", dir, prefix, cnt, extension);

            print!("{}/{}\r", cnt, self.total_frames);
            std::io::stdout().flush().unwrap();

            ascii_image.output_to_file(path, self.px);

            // save all files to mp4 and remove each file one added to the mp4

            // remove dir if it's empty
            // this should be empty because all the files should be removed
            if std::fs::read_dir(dir).unwrap().count() == 0 {
                std::fs::remove_dir(dir).unwrap();
            }
        });
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
            ascii_image.output_to_stdout();
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
