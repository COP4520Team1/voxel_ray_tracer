use image::{Rgba, RgbaImage};
use rayon::iter::plumbing::bridge;
use rayon::iter::plumbing::Producer;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::path::Path;
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
        let pixels = (0..size)
            .map(|_| AtomicU32::new(0))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn pixel_mut(&self, x: usize, y: usize) -> &AtomicU32 {
        let index = y * self.width + x;
        &self.pixels[index]
    }
}

impl<'b> IntoParallelIterator for &'b Framebuffer {
    type Iter = ParIter<'b>;
    type Item = PixelRef<'b>;

    fn into_par_iter(self) -> Self::Iter {
        ParIter { buffer: self }
    }
}

pub fn export_image(fb: Framebuffer, path: impl AsRef<Path>) -> image::ImageResult<()> {
    let mut img = RgbaImage::new(fb.width as u32, fb.height as u32);
    // Copy pixel at x, y from Framebuffer into image
    for x in 0..fb.width {
        for y in 0..fb.height {
            // Get RGBA value from pixels - first byte is R, second is G, etc
            let current_pixel = fb.pixel_mut(x, y).load(Ordering::Relaxed);
            let r_byte = (current_pixel >> 24) as u8;
            let g_byte = ((current_pixel & 0x00FF0000) >> 16) as u8;
            let b_byte = ((current_pixel & 0x0000FF00) >> 8) as u8;
            let a_byte = (current_pixel & 0x000000FF) as u8;
            img.put_pixel(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                Rgba([r_byte, g_byte, b_byte, a_byte]),
            );
        }
    }

    img.save(path).unwrap();

    Ok(())
}

pub struct Iter<'b> {
    buffer: &'b Framebuffer,
    start: usize,
    end: usize,
}

impl<'b> Iterator for Iter<'b> {
    type Item = PixelRef<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let color = self.buffer.pixels.get(self.start)?;

        let pixel = PixelRef {
            x: self.start % self.buffer.width,
            y: self.start / self.buffer.width,
            value: color,
        };

        // increment for next iteration
        self.start += 1;

        Some(pixel)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len: usize = self.end - self.start;
        (len, Some(len))
    }
}

impl<'b> DoubleEndedIterator for Iter<'b> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        // decrement first since end is exclusive
        self.end -= 1;

        let color = self.buffer.pixels.get(self.end)?;

        let pixel = PixelRef {
            x: self.end % self.buffer.width,
            y: self.end / self.buffer.width,
            value: color,
        };

        Some(pixel)
    }
}

impl<'b> ExactSizeIterator for Iter<'b> {}

pub struct ParIter<'b> {
    buffer: &'b Framebuffer,
}

impl<'b> ParallelIterator for ParIter<'b> {
    type Item = PixelRef<'b>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'b> IndexedParallelIterator for ParIter<'b> {
    fn len(&self) -> usize {
        self.buffer.pixels.len()
    }

    fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(
        self,
        callback: CB,
    ) -> CB::Output {
        callback.callback(ParIterProducer {
            buffer: &self.buffer,
            start: 0,
            end: self.len(),
        })
    }
}

struct ParIterProducer<'b> {
    buffer: &'b Framebuffer,
    start: usize,
    end: usize,
}

impl<'b> Producer for ParIterProducer<'b> {
    type Item = PixelRef<'b>;
    type IntoIter = Iter<'b>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            buffer: self.buffer,
            start: self.start,
            end: self.end,
        }
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let mid = self.start + index;
        assert!(self.start < mid && mid < self.end, "Out of range!");

        (
            Self {
                buffer: self.buffer,
                start: self.start,
                end: mid,
            },
            Self {
                buffer: self.buffer,
                start: mid,
                end: self.end,
            },
        )
    }
}

pub struct PixelRef<'b> {
    pub x: usize,
    pub y: usize,
    pub value: &'b AtomicU32,
}
