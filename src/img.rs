use std::fs::File;
use std::path::Path;
use image::{self, ImageBuffer, Rgb};

pub struct Img {
    buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Img {
    pub fn new() -> Img {
        let mut buf = ImageBuffer::new(840, 480);
        buf.put_pixel(0, 0, Rgb([0 as u8, 0 as u8, 0 as u8]));
        Img{
            buf: buf,
        }
    }

    pub fn put(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        self.buf.put_pixel(x, y, Rgb([r, g, b]));
    }

    pub fn save(self) {
        let ref mut fout = File::create(&Path::new("out.png")).unwrap();
        let _ = image::ImageRgb8(self.buf).save(fout, image::PNG);
    }
}
