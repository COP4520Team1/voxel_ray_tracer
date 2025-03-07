use std::path::PathBuf;

pub struct Framebuffer {
    width: u32,
    height: u32,
    pixels: Box<[AtomicU32]>,
}
impl Framebuffer {
    // creates a vector of size width * height, converts it to box
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = (0..(width * height))
        .map(|_| AtomicU32::new(0))
        .collect::<Vec<_>>()
        .into_boxed_slice();
        Self { width, height, pixels }
    }
    pub fn export_image(fb: Framebuffer /*, path: impl Into<PathBuf> */) -> image::ImageResult<()> {
        // Create image from framebuffer
        let mut img = RgbImage::new(fb.width, fb.height);
    
        // Copy framebuffer into image
        //
        //
        //
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }
        // Save img to file path
        //
        return Ok(());
    }
    
    pub fn pixel_mut(&self, x: u32, y: u32) -> &AtomicU32 {
        let index = (y * self.width + x) as usize;
        return &(self.pixels[index]);
        //todo!();

    }
    
    fn into_image(self) -> ImageBuffer<()> {
        Framebuffer::export_image(self);
        return RgbImage::new(self.width, self.height);

    }  
}