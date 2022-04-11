use ffmpeg::codec::decoder::video::Video as VideoDecoder;
use ffmpeg::format::context::Input as ContextImage;
use ffmpeg::media::Type::Video as VideoType;
use ffmpeg::software::converter;
use ffmpeg::util::format::pixel::Pixel as PixelFormat;
use ffmpeg::util::frame::video::Video as VideoFrame;
use image::RgbImage;

pub struct VideoStream {
    input: ContextImage,
    stream: usize,
    decoder: VideoDecoder,
}

impl VideoStream {
    pub fn new(input: &str) -> Result<Self, ffmpeg::Error> {
        ffmpeg::init()?;

        let input = ffmpeg::format::input(&input)?;

        let stream = input
            .streams()
            .best(VideoType)
            .ok_or("Failed to get stream")
            .unwrap();
        let decoder = stream.codec().decoder().video()?;
        let stream = stream.index();

        Ok(Self {
            input,
            stream,
            decoder,
        })
    }

    pub fn iter(&mut self) -> Iter<'_> {
        Iter { inner: self }
    }
}

pub struct Iter<'a> {
    inner: &'a mut VideoStream,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Frame;
    fn next(&mut self) -> Option<Self::Item> {
        let packets = self.inner.input.packets().next();
        packets.as_ref()?;

        let (stream, packet) = packets.unwrap();
        if stream.index() != self.inner.stream {
            return self.next();
        }

        let mut output = VideoFrame::empty();

        match self.inner.decoder.decode(&packet, &mut output) {
            Ok(_) => {
                if output.format() != PixelFormat::None {
                    Some(Frame::new(output))
                } else {
                    self.next()
                }
            }

            Err(error) => {
                eprintln!("{}", error);
                None
            }
        }
    }
}

pub struct Frame {
    frame: VideoFrame,
}

impl Frame {
    fn new(frame: VideoFrame) -> Self {
        Self { frame }
    }

    pub fn width(&self) -> u32 {
        self.frame.width()
    }

    pub fn height(&self) -> u32 {
        self.frame.height()
    }

    pub fn as_rgb(&self) -> Option<RgbImage> {
        let vec = self.as_vec(3, PixelFormat::RGB24);
        vec.as_ref()?;

        RgbImage::from_raw(self.width(), self.height(), vec.unwrap())
    }

    fn convert(&self, format: PixelFormat) -> Option<VideoFrame> {
        let mut output = VideoFrame::empty();

        let converter = converter((self.width(), self.height()), self.frame.format(), format)
            .unwrap()
            .run(&self.frame, &mut output);

        if converter.is_ok() {
            return Some(output);
        }

        None
    }

    pub fn as_vec(&self, channels: u32, format: PixelFormat) -> Option<Vec<u8>> {
        let output = self.convert(format)?;

        let index = 0;
        let stride = output.stride(index);
        let width = (output.width() * channels) as usize;

        // If the stride and width are equal, just convert to a vec
        if stride == width {
            return Some(output.data(index).to_vec());
        }

        // If they aren't, because the data has some garbage at the end of each line, skip over it
        let mut offset = 0;
        let mut vec = Vec::with_capacity((self.width() * self.height() * channels) as usize);
        let data = output.data(index);

        while offset < data.len() {
            vec.extend_from_slice(&data[offset..offset + width]);
            offset += stride;
        }

        Some(vec)
    }
}
