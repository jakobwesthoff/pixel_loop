//! Canvas implementations for rendering pixels on different output targets
//!
//! A Canvas is a 2D grid of pixels that can be drawn to and rendered. It provides
//! methods for setting and getting pixel colors, as well as blitting (copying) regions
//! of pixels from one canvas to another.
//!
//! Furthermore it provides convenience methods for drawing certain shape
//! primitives (eg. rectangles).
//!
//! It is the goto abstraction for rendering pixels in the pixel_loop library.

#[cfg(feature = "crossterm")]
pub mod crossterm;
#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermCanvas;

pub mod in_memory;
pub use in_memory::InMemoryCanvas;

#[cfg(feature = "winit")]
pub mod pixels;
#[cfg(feature = "winit")]
pub use pixels::PixelsCanvas;

use crate::color::Color;
use crate::input::InputState;
use crate::PixelLoop;

use anyhow::{Context, Result};
use std::ops::Range;

/// Trait representing a basic canvas that can be drawn to.
///
/// A canvas provides basic pixel manipulation operations and blitting capabilities
/// for 2D graphics rendering as well as convenience methods for drawing shapes.
///
/// Different implementations of this trait exist to utilize different rendering
/// backends like an ansi terminal
/// ([CrosstermCanvas](crate::canvas::crossterm::CrosstermCanvas)), or a window
/// ([PixelsCanvas](crate::canvas::pixels::PixelsCanvas)).
pub trait Canvas {
    /// Get the width of the canvas in pixels
    fn width(&self) -> u32;
    /// Get the height of the canvas in pixels
    fn height(&self) -> u32;
    /// Set a range of pixels to a given [Color]
    fn set_range(&mut self, range: Range<usize>, color: &[Color]);
    /// Get a range of pixels as a slice of [Color]s
    fn get_range(&self, range: Range<usize>) -> &[Color];

    /// Blit a full input canvas to this canvas instance at a given position,
    /// optionally tinting the input canvas with a color.
    ///
    /// # Arguments
    /// * `src_canvas` - The source canvas to blit from
    /// * `dst_x` - The x position to blit the source canvas to
    /// * `dst_y` - The y position to blit the source canvas to
    /// * `tint` - An optional color to tint the source canvas with
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

    /// Blit only a rectangular region of the input canvas to this canvas instance at a given position,
    /// optionally tinting the input canvas with a color.
    /// If the source rectangle is partially out of view, only the visible part will be blitted.
    /// If the destination rectangle is partially out of view, only the visible part will be blitted.
    ///
    /// See also: [blit_rect](crate::canvas::Canvas::blit_rect)
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
            self.clip_rect(dst_x, dst_y, width, height)
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

    /// Get the color of a specific pixel at a given position
    fn get(&self, x: u32, y: u32) -> &Color {
        let i = (y * self.width() + x) as usize;
        let color_slice = self.get_range(i..i + 1);
        // @TODO: Check if clone happens here
        &color_slice[0]
    }

    /// Get the color of a specific pixel at a given position, if it is in bounds of the canvas.
    ///
    /// # Returns
    /// * `Some(&Color)` - If the position is in bounds
    /// * `None` - If the position is out of bounds
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
    /// Check if a specific pixel at a given position is out of bounds (empty)
    /// or has the specified color.
    ///
    /// This method primarily can be used for pixel based collision detection.
    fn is_empty_or_color(&self, x: i64, y: i64, color: &Color) -> bool {
        self.maybe_get(x, y).map(|c| c == color).unwrap_or(true)
    }

    /// Set the color of a specific pixel at a given position
    fn set(&mut self, x: u32, y: u32, color: &Color) {
        let i = (y * self.width() + x) as usize;
        self.set_range(i..i + 1, std::slice::from_ref(color));
    }

    /// Clip a rectangle to the bounds of the canvas.
    ///
    /// # Returns
    /// * `Some((u32, u32, u32, u32))` - If the rectangle is partially or fully in view
    /// * `None` - If the rectangle is completely out of view
    fn clip_rect(&self, x: i64, y: i64, width: u32, height: u32) -> Option<(u32, u32, u32, u32)> {
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

    /// Clear (fill) the whole canvas with a specific color
    fn clear_screen(&mut self, color: &Color) {
        self.filled_rect(0, 0, self.width(), self.height(), color)
    }

    /// Draw a filled rectangle at a given position with a given width and height
    fn filled_rect(&mut self, sx: i64, sy: i64, width: u32, height: u32, color: &Color) {
        if let Some((sx, sy, width, height)) = self.clip_rect(sx, sy, width, height) {
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

/// Trait representing a canvas that can be rendered to a display target, like a
/// window or terminal.
///
/// Not necessarily all canvas implementations need to implement this trait, as
/// there might be internally represented pixel data within a canvas, like
/// [InMemoryCanvas](crate::canvas::in_memory::InMemoryCanvas), which can be
/// used to hold loaded images.
pub trait RenderableCanvas: Canvas {
    /// Renders the current state of the canvas.
    ///
    /// # Returns
    /// A Result indicating whether the rendering was successful
    fn render(&mut self) -> Result<()>;

    /// Converts physical screen coordinates to canvas coordinates.
    ///
    /// # Arguments
    /// * `x` - The physical x coordinate
    /// * `y` - The physical y coordinate
    ///
    /// # Returns
    /// Some((x, y)) with canvas coordinates if conversion is possible,
    /// None otherwise
    fn physical_pos_to_canvas_pos(&self, x: f64, y: f64) -> Option<(u32, u32)>;

    // @TODO: Not all surfaces are resizable, so this should be in someway optional?
    /// Resizes the rendering surface.
    ///
    /// # Arguments
    /// * `width` - The new width
    /// * `height` - The new height
    fn resize_surface(&mut self, width: u32, height: u32);

    // @TODO: Are all canvases resizable?
    /// Resizes the canvas.
    ///
    /// # Arguments
    /// * `width` - The new width
    /// * `height` - The new height
    fn resize(&mut self, width: u32, height: u32);

    fn run<State: 'static, InputImpl: InputState + 'static>(
        mut pixel_loop: PixelLoop<State, InputImpl, Self>,
    ) -> !
    where
        Self: Sized,
    {
        pixel_loop.begin().unwrap();
        loop {
            pixel_loop
                .next_loop()
                .context("run next pixel loop")
                .unwrap()
        }
        // pixel_loop.finish().unwrap();
    }
}
