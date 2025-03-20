use std::path::PathBuf;
use image::{RgbImage, Rgb};
use std::sync::atomic::AtomicU32;

pub struct Framebuffer {
    width: usize,
    height: usize,
    pixels: Box<[AtomicU32]>,
}
impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let size = width * height;
        let pixels = (0..size).map(|_| AtomicU32::new(0)).collect::<Vec<_>>().into_boxed_slice();
        
        Self { width, height, pixels }
    }
    pub fn pixel_mut(&self, x: usize, y: usize) -> &AtomicU32 {
        let index = y * self.width + x;
        &self.pixels[index]
    }
    pub fn export_image(fb: Framebuffer, path: impl Into<PathBuf>) -> image::ImageResult<()> {
        let mut img = RgbImage::new(fb.width, fb.height);
    
        // Copy pixel at x, y from Framebuffer into image
        for x in fb.width {
            for y in fb.height {
                // Get RGB value from pixels 
                img.put_pixel(x, y, Rgb(fb.pixel_mut(x, y).load()));
            }
        }
        
        img.save("render.png").unwrap();
    
        Ok(())
    }
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

