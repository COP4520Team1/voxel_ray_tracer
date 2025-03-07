use std::path::PathBuf;

pub struct Framebuffer {}

pub fn export_image(fb: Framebuffer, path: impl Into<PathBuf>) -> image::ImageResult<()> {
    todo!()
}

pub struct FramebufferIter<'fb> {
    fb: &'fb Framebuffer,
    i: usize,
}

impl<'fb> Iterator for FramebufferIter<'fb> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
