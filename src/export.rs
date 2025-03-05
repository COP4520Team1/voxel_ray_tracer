use std::path::PathBuf;

pub struct Framebuffer {
    width: u32,
    height: u32,
    // vector of all the pixels
    pixels: Box<[AtomicU32]>,
}
impl Framebuffer {
    pub fn export_image(fb: Framebuffer, path: impl Into<PathBuf>) -> image::ImageResult<()> {
        // Create image from framebuffer
        let mut img = RgbImage::new(fb.width, fb.height);
    
        // Copy framebuffer into image
        //
        //
        //
    
        return img;
    }
    
    pub fn pixel_mut(&self, x: usize, y: usize) -> &AtomicU32 {
        return self.pixels[y * self.width + x];
    
    }
    
    fn into_image(self) -> Image {
        todo!();
    }    
}