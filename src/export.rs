use std::path::PathBuf;

pub struct Framebuffer {
    width: u32,
    height: u32, 
}

pub fn export_image(fb: Framebuffer, path: impl Into<PathBuf>) -> image::ImageResult<()> {
    let width = fb.width;
    let height = fb.height;
    let mut img = RgbImage::new(width, height);




    return img;
}

pub fn pixel_mut(&self, x: usize, y: usize) -> &AtomicU32 {

}

fn into_image(self) -> Image {

}