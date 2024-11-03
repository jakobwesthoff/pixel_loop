use std::ops::Range;

use anyhow::anyhow;
use anyhow::Result;

use crate::color::Color;

use super::Canvas;

pub struct InMemoryCanvas {
    buffer: Vec<Color>,
    width: u32,
    height: u32,
}

impl InMemoryCanvas {
    pub fn new(width: u32, height: u32, color: &Color) -> Self {
        Self {
            buffer: vec![color.clone(); (width * height) as usize],
            width,
            height,
        }
    }

    pub fn from_in_memory_image(bytes: &[u8]) -> Result<Self> {
        use stb_image::image;
        use stb_image::image::LoadResult::*;
        match image::load_from_memory(bytes) {
            Error(msg) => return Err(anyhow!("Could not load image from memory: {msg}")),
            ImageF32(_) => return Err(anyhow!("Could not load hdr image from memory")),
            ImageU8(image) => {
                if image.depth != 3 {
                    return Err(anyhow!(
                        "Could not load image with depth != 3. It has {depth}",
                        depth = image.depth
                    ));
                }

                let mut buffer: Vec<Color> = Vec::with_capacity(image.width * image.height);
                for i in (0..image.width * image.height * image.depth).step_by(image.depth) {
                    buffer.push(Color::from_rgb(
                        image.data[i],
                        image.data[i + 1],
                        image.data[i + 2],
                    ))
                }

                return Ok(Self {
                    width: image.width as u32,
                    height: image.height as u32,
                    buffer,
                });
            }
        }
    }
}

impl Canvas for InMemoryCanvas {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn set_range(&mut self, range: Range<usize>, color: &[Color]) {
        self.buffer[range].copy_from_slice(color);
    }

    fn get_range(&self, range: Range<usize>) -> &[Color] {
        &self.buffer[range]
    }
}
