use std::path::PathBuf;
use image::{RgbaImage, Rgba};
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

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
    
}
pub fn export_image(fb: Framebuffer, path: impl Into<PathBuf>) -> image::ImageResult<()> {
    let mut img = RgbaImage::new(fb.width as u32, fb.height as u32);
    // Copy pixel at x, y from Framebuffer into image
    for x in (0..fb.width) {
        for y in (0..fb.height) {
            // Get RGBA value from pixels - first byte is R, second is G, etc
            let current_pixel = fb.pixel_mut(x, y).load(Ordering::Relaxed);
            let r_byte = (current_pixel >> 24) as u8;
            let g_byte = ((current_pixel & 0x00FF0000) >> 16) as u8;
            let b_byte = ((current_pixel & 0x0000FF00) >> 8) as u8;
            let a_byte = (current_pixel & 0x000000FF) as u8;
            img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), Rgba([r_byte, g_byte, b_byte, a_byte]));
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
        todo!();
    }
}

