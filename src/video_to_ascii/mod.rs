use image::imageops::{resize, FilterType};
use image::GenericImageView;
use std::io::Write;
use std::process::Command;
use terminal_size::terminal_size;

mod videostream;
use videostream::*;

mod ascii_image;
use ascii_image::*;

#[derive(PartialEq, Clone)]
pub enum Flag {
    Ascii,
    Color,
    Invert,
}

pub struct AsciiVideo {
    stream: VideoStream,
    output: Option<String>,
    dimensions: (Option<u32>, Option<u32>, u32),
    filter: FilterType,
    flags: Vec<Flag>,
    total_frames: usize,
}

impl AsciiVideo {
    pub fn new(
        io: (&str, Option<&str>),
        dimensions: (Option<u32>, Option<u32>, u32),
        filter: &str,
        flags: Vec<Flag>,
    ) -> Self {
        let mut video = VideoStream::new(io.0).unwrap();
        let total_frames = video.iter().fold(0, |acc, _| acc + 1);
        let video = VideoStream::new(io.0).unwrap();

        let filter = match filter {
            "triangle" => FilterType::Triangle,
            "catmullrom" => FilterType::CatmullRom,
            "lanczos3" => FilterType::Lanczos3,
            _ => FilterType::Nearest,
        };

        Self {
            stream: video,
            output: io.1.map(Into::into),
            dimensions,
            filter,
            flags,
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
        let dir = format!(".tmp_video_to_ascii-{}", std::process::id());
        let dir = dir.as_str();

        let prefix = self.output.as_ref().unwrap().split('.').next().unwrap();
        if !std::path::Path::new(dir).exists() {
            std::fs::create_dir(dir).unwrap();
        }

        for (i, frame) in self.stream.iter().enumerate() {
            let img = frame.as_rgb().unwrap();
            let img = match Self::width_and_height(
                self.dimensions.0,
                self.dimensions.1,
                self.output.clone(),
                &img,
            ) {
                Some((width, height)) => resize(&img, width, height, self.filter),
                None => img,
            };

            let path = format!("{}/{}_{}.png", dir, prefix, i);

            print!("{} / {}\r", i, self.total_frames);
            std::io::stdout().flush().unwrap();

            AsciiImage::new(img).output_to_file(path, self.dimensions.2, self.flags.clone());
        }
        // save all files to mp4 and remove each file one added to the mp4
        // TODO make this work without Command
        Command::new("ffmpeg")
            .args(&[
                "-i",
                &format!("{}/{}_%d.png", dir, prefix),
                self.output.as_ref().unwrap(),
            ])
            .output()
            .expect("failed to execute process");

        std::fs::remove_dir_all(dir).unwrap();
    }

    fn width_and_height<I: GenericImageView>(
        width: Option<u32>,
        height: Option<u32>,
        output: Option<String>,
        image: &I,
    ) -> Option<(u32, u32)> {
        if let Some(width) = width {
            // width and height
            if let Some(height) = height {
                return Some((width, height));
            }
            // width
            let height = width as f64 / image.width() as f64 * image.height() as f64;
            return Some((width, height as u32));
        }

        // height
        if let Some(height) = height {
            let width = height as f64 / image.height() as f64 * image.width() as f64;
            return Some((width as u32, height));
        }

        // all empty
        if width.is_none() && height.is_none() && output.is_none() {
            let (width, height) = terminal_size().unwrap();
            return Some((width.0 as u32, height.0 as u32));
        }

        None
    }

    fn stdout_output(&mut self) {
        for frame in self.stream.iter() {
            let img = frame.as_rgb().unwrap();
            let width_and_height = Self::width_and_height(
                self.dimensions.0,
                self.dimensions.1,
                self.output.clone(),
                &img,
            );
            let img = match width_and_height {
                Some((width, height)) => resize(&img, width, height, self.filter),
                None => img,
            };

            let ascii_image = AsciiImage::new(img);
            ascii_image.output_to_stdout(self.flags.clone());
            print!("\x1B[1;1H"); // move curser
        }
    }
}
