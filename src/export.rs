use std::path::PathBuf;
use image::{RgbImage, Rgb};
use std::sync::atomic::AtomicU32;

pub struct Framebuffer {
    width: u32,
    height: u32,
    pixels: Box<[AtomicU32]>,
}

pub fn export_image(fb: Framebuffer, path: impl Into<PathBuf>) -> image::ImageResult<()> {
    let mut img = RgbImage::new(fb.width, fb.height);

    // Copy pixel at x, y from Framebuffer into image
    for x in fb.width {
        for y in fb.height {
            // Get RGB value from pixels 
            // let p: [u32, 3] = fb.pixels[y * x + x];
            img.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }
    
    img.save("render.png").unwrap();

    Ok(())
}

pub struct FramebufferIter<'fb> {
    fb: &'fb Framebuffer,
    i: usize,
}

impl<'fb> Iterator for FramebufferIter<'fb> {
    type Item = (); // TODO: change to AtomicU32

    fn next(&mut self) -> Option<Self::Item> {
        todo()!;
    }
}

