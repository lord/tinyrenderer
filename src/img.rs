use std::fs::File;
use std::path::Path;
use image::{self, ImageBuffer, Rgb};

pub struct Img {
    buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
    w: u32,
    h: u32,
}

impl Img {
    pub fn new(w: u32, h: u32) -> Img {
        let mut buf = ImageBuffer::new(w, h);
        buf.put_pixel(0, 0, Rgb([0 as u8, 0 as u8, 0 as u8]));
        Img{
            buf: buf,
            w: w,
            h: h,
        }
    }

    pub fn put(&mut self, x: u32, y: u32, color: (u8, u8, u8)) {
        if x < self.w && y < self.h {
            self.buf.put_pixel(y, x, Rgb([color.0, color.1, color.2]));
        }
    }

    pub fn save(self) {
        let ref mut fout = File::create(&Path::new("out.png")).unwrap();
        let _ = image::ImageRgb8(self.buf).save(fout, image::PNG);
    }
}
