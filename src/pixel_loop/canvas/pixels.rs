//! Window-based canvas implementation using the pixels crate.
//!
//! This module provides a canvas implementation that renders directly to a window
//! using the pixels crate for hardware-accelerated rendering. It requires the
//! "pixels" feature to be enabled.

use super::{Canvas, RenderableCanvas};
use crate::color::{Color, ColorAsByteSlice};
use crate::input::PixelsInputState;
use crate::NextLoopState;
use anyhow::{Context, Result};
use pixels::{Pixels, SurfaceTexture};
use std::ops::Range;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

/// Context winit window-related resources.
struct WinitContext {
    event_loop: EventLoop<()>,
    input_helper: WinitInputHelper,
    window: Window,
}

/// A canvas implementation that renders to a window using the pixels crate.
///
/// This canvas provides hardware-accelerated rendering to a window surface
/// using the pixels crate. It handles pixel data conversion between the
/// internal Color type and the RGBA byte format required by pixels.
///
/// # Example
///
/// ```
/// let canvas = PixelsCanvas::new(640, 480, "pixel loop", false)?;
/// ```
pub struct PixelsCanvas {
    /// The scale factor of the canvas supplied by the user to create a more
    /// "blocky" pixel feeling.
    user_scale_factor: u32,
    /// The winit window context
    context: Option<WinitContext>,
    /// The underlying pixels instance for window rendering
    pixels: Pixels,
    /// The width of this canvas during the last loop
    last_loop_width: u32,
    /// The height of this canvas during the last loop
    last_loop_height: u32,
}

impl PixelsCanvas {
    /// Creates a new window-based canvas using the pixels crate as a backend.
    ///
    /// # Arguments
    /// * `width` - The width of the canvas in pixels
    /// * `height` - The height of the canvas in pixels
    /// * `scale_factor` - The scale factor of real window pixels to rendering canvas pixels
    /// * `title` - The title of the window
    /// * `resizable` - Whether the window should be resizable (This implies, that the pixel canvas size can change)
    pub fn new(
        width: u32,
        height: u32,
        scale_factor: Option<u32>,
        title: &str,
        resizable: bool,
    ) -> Result<Self> {
        let event_loop = EventLoop::new();
        let input_helper = WinitInputHelper::new();
        let window = {
            // This is the size, that we essentially want to use as window size,
            // if the screen is rendered at 100% scale.
            // It might not be the resolution, that is in the end actually
            // rendered.
            // And that may be again different from the size of the pixels
            // buffer, as this is scaled by the user supplied scale_factor as
            // well.
            let logical_window_size = LogicalSize::new(width as f64, height as f64);
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(logical_window_size)
                .with_min_inner_size(logical_window_size)
                .with_resizable(resizable)
                .build(&event_loop)?
        };

        let context = WinitContext {
            event_loop,
            input_helper,
            window,
        };

        // This is the actual size of the window in pixels, that is rendered.
        // Scaled by by the window.scale_factor
        let physical_dimensions = context.window.inner_size();
        let surface_texture = SurfaceTexture::new(
            physical_dimensions.width,
            physical_dimensions.height,
            &context.window,
        );

        // This is the size of the pixels buffer, that is based on the logical
        // (non system scaled) window size and the user supplied scale_factor
        let scaled_buffer_width = width / scale_factor.unwrap_or(1);
        let scaled_buffer_height = height / scale_factor.unwrap_or(1);
        let pixels = Pixels::new(scaled_buffer_width, scaled_buffer_height, surface_texture)
            .context("create pixels surface")?;

        Ok(Self {
            user_scale_factor: scale_factor.unwrap_or(1),
            context: Some(context),
            pixels,
            last_loop_height: 0, // Zero initialized to cause initial update
            last_loop_width: 0,  // Zero initialized to cause initial update
        })
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
    type Input = PixelsInputState;

    // @TODO: Move to input when handling mouse control there
    // fn physical_pos_to_canvas_pos(&self, x: f64, y: f64) -> Option<(u32, u32)> {
    //     if let Ok((x, y)) = self.pixels.window_pos_to_pixel((x as f32, y as f32)) {
    //         Some((x as u32, y as u32))
    //     } else {
    //         None
    //     }
    // }

    fn render(&mut self) -> Result<()> {
        self.pixels
            .render()
            .context("letting pixels lib blit to screen")?;
        Ok(())
    }

    fn resize_surface(&mut self, width: u32, height: u32, window_scale_factor: Option<f64>) {
        self.pixels
            .resize_surface(width, height)
            .expect("to be able to resize surface");

        // First scale the display size by the window scale factor, then scale
        // by the user factor as well.
        let display_scaled_width = (width as f64 / window_scale_factor.unwrap_or(1.0)) as u32;
        let display_scaled_height = (height as f64 / window_scale_factor.unwrap_or(1.0)) as u32;
        let user_scaled_width = display_scaled_width / self.user_scale_factor;
        let user_scaled_height = display_scaled_height / self.user_scale_factor;
        self.pixels
            .resize_buffer(user_scaled_width, user_scaled_height)
            .expect("to be able to resize buffer");
    }

    /// Run the pixel loop, handling events and rendering.
    ///
    /// This implementation overrides the generic pixel_loop implementation, to
    /// handle the winit event_loop properly.
    fn run<State: 'static>(mut pixel_loop: crate::PixelLoop<State, Self>) -> !
    where
        Self: Sized,
    {
        // We may take the context here, as we are never returning from this
        // function again.
        let context = pixel_loop.canvas.take_context();

        pixel_loop.begin().context("initialize pixel_loop").unwrap();
        let mut exit_code = 0i32;
        context.event_loop.run(move |event, _, control_flow| {
            pixel_loop.input_state.handle_new_event(&event);
            match event {
                Event::MainEventsCleared => {
                    let next = pixel_loop
                        .next_loop()
                        .context("run next pixel loop")
                        .unwrap();
                    if let NextLoopState::Exit(code) = next {
                        exit_code = code;
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    // Track last communicated canvas size
                    pixel_loop.canvas.last_loop_width = pixel_loop.canvas.width();
                    pixel_loop.canvas.last_loop_height = pixel_loop.canvas.height();
                }
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    // Handle window resize events and correct buffer and
                    // surface sizes
                    WindowEvent::Resized(physical_size) => {
                        pixel_loop.canvas.resize_surface(
                            physical_size.width,
                            physical_size.height,
                            Some(context.window.scale_factor()),
                        );
                    }
                    WindowEvent::CloseRequested => {
                        exit_code = 0;
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => {}
                },
                Event::LoopDestroyed => {
                    pixel_loop
                        .finish(exit_code)
                        .context("finish pixel loop")
                        .unwrap();
                }
                _ => {}
            }
        });
    }

    fn did_resize(&self) -> Option<(u32, u32)> {
        if self.last_loop_width != self.width() || self.last_loop_height != self.height() {
            Some((self.width(), self.height()))
        } else {
            None
        }
    }
}
