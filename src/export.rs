use std::path::PathBuf;

pub struct Framebuffer {
    width: usize,
    height: usize,
    pixels: Box<[AtomicU32]>,
}
impl Framebuffer {
    // creates a vector of size width * height, converts it to box
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = (0..(width * height))
        .map(|_| AtomicU32::new(0))
        .collect::<Vec<_>>()
        .into_boxed_slice();
        Self { width, height, pixels }

    }
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
        // return self.pixels[y * self.width + x];
        todo!();

    }
    
    fn into_image(self) -> Image {
        todo!();
    }    
}