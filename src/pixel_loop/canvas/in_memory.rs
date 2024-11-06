//! In-memory canvas implementation with image loading capabilities.
//!
//! This module provides a basic canvas implementation that stores pixel data in memory.
//!
//! It can be used to load image data from disk if the `stb-image` feature is
//! enabled.

use super::Canvas;
use crate::color::Color;
use anyhow::anyhow;
use anyhow::Result;
use std::ops::Range;

/// A canvas implementation that stores pixel data in memory.
///
/// This canvas provides basic pixel manipulation operations and can be used
/// to load and manipulate images in memory.
pub struct InMemoryCanvas {
    /// The pixel buffer storing all colors
    buffer: Vec<Color>,
    /// Width of the canvas in pixels
    width: u32,
    /// Height of the canvas in pixels
    height: u32,
}

impl InMemoryCanvas {
    /// Creates a new blank canvas with the specified dimensions and background color.
    ///
    /// # Arguments
    /// * `width` - The width of the canvas in pixels
    /// * `height` - The height of the canvas in pixels
    /// * `color` - The initial color to fill the canvas with
    ///
    /// # Examples
    /// ```
    /// use pixel_loop::{canvas::InMemoryCanvas, color::Color};
    ///
    /// let canvas = InMemoryCanvas::new(640, 480, &Color::from_rgb(0, 0, 0));
    /// ```
    pub fn new(width: u32, height: u32, color: &Color) -> Self {
        Self {
            buffer: vec![color.clone(); (width * height) as usize],
            width,
            height,
        }
    }

    /// Creates a new canvas by loading an image from a memory buffer.
    ///
    /// This method supports all image formats (non HDR), that can be read by the
    /// [stb_image](https://github.com/nothings/stb/blob/master/stb_image.h)
    /// library.
    ///
    /// Only available if the `stb-image` feature is enabled.
    ///
    /// # Arguments
    /// * `bytes` - Raw image bytes to load
    ///
    /// # Returns
    /// * `Ok(InMemoryCanvas)` - Successfully loaded canvas
    /// * `Err` - If the image couldn't be loaded or has an unsupported format
    ///
    /// # Errors
    /// Returns an error if:
    /// * The image data is invalid or corrupted
    /// * The image is HDR (32-bit float)
    /// * The image depth is not 3 (RGB)
    ///
    /// # Examples
    /// ```
    /// use pixel_loop::canvas::InMemoryCanvas;
    ///
    /// let image_bytes = std::fs::read("example.jpg").unwrap();
    /// let canvas = InMemoryCanvas::from_in_memory_image(&image_bytes)?;
    /// ```
    #[cfg(feature = "stb-image")]
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
