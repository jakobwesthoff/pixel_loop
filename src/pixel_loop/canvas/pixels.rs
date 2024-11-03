use crate::color::{Color, ColorAsByteSlice};
use anyhow::{Context, Result};
use pixels::Pixels;
use std::ops::Range;

use super::{Canvas, RenderableCanvas};

pub struct PixelsCanvas {
    pixels: Pixels,
}

impl PixelsCanvas {
    pub fn new(pixels: Pixels) -> Self {
        Self { pixels }
    }
}

impl Canvas for PixelsCanvas {
    fn width(&self) -> u32 {
        self.pixels.texture().width()
    }
    fn height(&self) -> u32 {
        self.pixels.texture().height()
    }

    fn get_range(&self, range: Range<usize>) -> &[Color] {
        let byte_range = range.start * 4..range.end * 4;
        let buf = self.pixels.frame();
        let byte_slice = &buf[byte_range];
        Color::from_bytes(byte_slice)
    }

    fn set_range(&mut self, range: Range<usize>, colors: &[Color]) {
        let byte_range = range.start * 4..range.end * 4;
        let buf = self.pixels.frame_mut();
        buf[byte_range].copy_from_slice(colors.as_byte_slice())
    }
}

impl RenderableCanvas for PixelsCanvas {
    fn physical_pos_to_canvas_pos(&self, x: f64, y: f64) -> Option<(u32, u32)> {
        if let Ok((x, y)) = self.pixels.window_pos_to_pixel((x as f32, y as f32)) {
            Some((x as u32, y as u32))
        } else {
            None
        }
    }

    fn render(&mut self) -> Result<()> {
        self.pixels
            .render()
            .context("letting pixels lib blit to screen")?;
        Ok(())
    }

    fn resize_surface(&mut self, width: u32, height: u32) {
        self.pixels
            .resize_surface(width, height)
            .expect("to be able to resize surface");
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.pixels
            .resize_buffer(width, height)
            .expect("to be able to resize buffer");
    }
}
