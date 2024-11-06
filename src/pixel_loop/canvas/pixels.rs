//! Window-based canvas implementation using the pixels crate.
//!
//! This module provides a canvas implementation that renders directly to a window
//! using the pixels crate for hardware-accelerated rendering. It requires the
//! "winit" feature to be enabled.

use super::{Canvas, RenderableCanvas};
use crate::color::{Color, ColorAsByteSlice};
use crate::input::InputState;
use crate::winit::WinitContext;
use anyhow::{Context, Result};
use pixels::Pixels;
use std::ops::Range;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

/// A canvas implementation that renders to a window using the pixels crate.
///
/// This canvas provides hardware-accelerated rendering to a window surface
/// using the pixels crate. It handles pixel data conversion between the
/// internal Color type and the RGBA byte format required by pixels.
///
/// # Example
///
/// The creation of the canvas should always be done using the corresponding factory functions to embed it into a proper window initialization.
/// In this case it is done using the winit wrapper of the pixel_loop library:
///
/// ```
/// use pixel_loop::winit
///
/// let context = winit::init_window("pixel loop", 640, 480, false)?;
/// let canvas = winit::init_pixels(&context, 640, 480)?
/// ```
pub struct PixelsCanvas {
    /// The underlying pixels instance for window rendering
    pixels: Pixels,
    context: Option<WinitContext>,
}

impl PixelsCanvas {
    /// Creates a new window-based canvas from a pixels instance.
    ///
    /// # Arguments
    /// * `pixels` - A configured pixels instance for rendering
    ///
    /// # Notes
    /// This method should not be called directly, but instead use the
    /// `init_pixels` factory function provided by the winit module.
    /// This is to ensure proper initialization of the window and event loop.
    /// See the example in [PixelsCanvas](crate::canvas::pixels::PixelsCanvas) for more details.
    pub fn new(context: WinitContext, pixels: Pixels) -> Self {
        Self { context: Some(context), pixels }
    }
}

impl PixelsCanvas {
    fn take_context(&mut self) -> WinitContext {
        self.context.take().unwrap()
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

    fn run<State: 'static, InputImpl: InputState + 'static>(
        mut pixel_loop: crate::PixelLoop<State, InputImpl, Self>,
    ) -> !
    where
        Self: Sized,
    {
        let context = pixel_loop.canvas.take_context();
        context.event_loop.run(move |event, _, control_flow| match event {
            Event::MainEventsCleared => {
                pixel_loop
                    .next_loop()
                    .context("run next pixel loop")
                    .unwrap();
            }
            Event::WindowEvent {
                event: win_event, ..
            } => match win_event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        });
    }
}
