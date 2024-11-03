#[cfg(feature = "crossterm")]
pub mod crossterm;
#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermCanvas;

#[cfg(feature = "image-load")]
pub mod in_memory;
#[cfg(feature = "image-load")]
pub use in_memory::InMemoryCanvas;

#[cfg(feature = "winit")]
pub mod pixels;
#[cfg(feature = "winit")]
pub use pixels::PixelsCanvas;

use crate::color::Color;

use anyhow::Result;
use std::ops::Range;

pub trait Canvas {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_range(&mut self, range: Range<usize>, color: &[Color]);
    fn get_range(&self, range: Range<usize>) -> &[Color];

    fn blit<C: Canvas>(&mut self, src_canvas: &C, dst_x: i64, dst_y: i64, tint: Option<&Color>) {
        self.blit_rect(
            src_canvas,
            0,
            0,
            src_canvas.width(),
            src_canvas.height(),
            dst_x,
            dst_y,
            tint,
        )
    }

    fn blit_rect<C: Canvas>(
        &mut self,
        src_canvas: &C,
        src_x: u32,
        src_y: u32,
        width: u32,
        height: u32,
        dst_x: i64,
        dst_y: i64,
        tint: Option<&Color>,
    ) {
        if let Some((norm_dst_x, norm_dst_y, norm_width, norm_height)) =
            self.normalize_rect(dst_x, dst_y, width, height)
        {
            for y in 0..norm_height {
                let src_start = (((src_y + y) * src_canvas.width()) + src_x) as usize;
                let src_end = src_start + u32::min(width, norm_width) as usize;
                let dst_start = (((norm_dst_y + y) * self.width()) + norm_dst_x) as usize;
                let dst_end = dst_start + norm_width as usize;
                let row = src_canvas.get_range(src_start..src_end);

                if let Some(tint) = tint {
                    self.set_range(
                        dst_start..dst_end,
                        &row.iter()
                            .map(|c| {
                                Color::from_rgb(
                                    (c.r as usize * tint.r as usize / 255 as usize) as u8,
                                    (c.g as usize * tint.g as usize / 255 as usize) as u8,
                                    (c.b as usize * tint.b as usize / 255 as usize) as u8,
                                )
                            })
                            .collect::<Vec<Color>>(),
                    );
                } else {
                    self.set_range(dst_start..dst_end, row);
                }
            }
        }
    }

    fn get(&self, x: u32, y: u32) -> &Color {
        let i = (y * self.width() + x) as usize;
        let color_slice = self.get_range(i..i + 1);
        // @TODO: Check if clone happens here
        &color_slice[0]
    }

    fn maybe_get(&self, x: i64, y: i64) -> Option<&Color> {
        if x < 0 || y < 0 || x >= self.width() as i64 || y >= self.height() as i64 {
            // Out of view
            None
        } else {
            Some(self.get(x as u32, y as u32))
        }
    }

    // @TODO: Not ideal: It would be better if the canvas knew its "empty"
    // (clear) color and could be asked `is_empty`.
    fn is_empty_or_color(&self, x: i64, y: i64, color: &Color) -> bool {
        self.maybe_get(x, y).map(|c| c == color).unwrap_or(true)
    }

    fn set(&mut self, x: u32, y: u32, color: &Color) {
        let i = (y * self.width() + x) as usize;
        self.set_range(i..i + 1, std::slice::from_ref(color));
    }

    fn normalize_rect(
        &self,
        x: i64,
        y: i64,
        width: u32,
        height: u32,
    ) -> Option<(u32, u32, u32, u32)> {
        let width = width as i64;
        let height = height as i64;
        if x < -width || y < -height || x >= self.width() as i64 || y >= self.height() as i64 {
            // Completely out of view
            None
        } else {
            let norm_x = i64::max(0, x);
            let norm_y = i64::max(0, y);
            let norm_width = i64::min(width - (norm_x - x), self.width() as i64 - norm_x - 1);
            let norm_height = i64::min(height - (norm_y - y), self.height() as i64 - norm_y - 1);
            Some((
                norm_x as u32,
                norm_y as u32,
                norm_width as u32,
                norm_height as u32,
            ))
        }
    }

    fn clear_screen(&mut self, color: &Color) {
        self.filled_rect(0, 0, self.width(), self.height(), color)
    }

    fn filled_rect(&mut self, sx: i64, sy: i64, width: u32, height: u32, color: &Color) {
        if let Some((sx, sy, width, height)) = self.normalize_rect(sx, sy, width, height) {
            let color_row = vec![color.clone(); width as usize];
            for y in sy..sy + height {
                self.set_range(
                    (y * self.width() + sx) as usize..(y * self.width() + sx + width) as usize,
                    color_row.as_slice(),
                );
            }
        }
    }
}

pub trait RenderableCanvas: Canvas {
    fn render(&mut self) -> Result<()>;
    fn physical_pos_to_canvas_pos(&self, x: f64, y: f64) -> Option<(u32, u32)>;
    fn resize_surface(&mut self, width: u32, height: u32);
    fn resize(&mut self, width: u32, height: u32);
}
