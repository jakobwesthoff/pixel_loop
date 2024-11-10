//! Terminal-based canvas implementation using the crossterm crate.
//!
//! This module provides a canvas implementation that renders to the terminal
//! using crossterm for colored output. It requires the "crossterm" feature
//! to be enabled. The implementation uses Unicode half blocks for rendering
//! and supports frame rate limiting.

use super::{Canvas, RenderableCanvas};
use crate::color::Color;
use crate::input::CrosstermInputState;
use anyhow::Result;
use crossterm::event::Event;
use crossterm::style::{self, Print, SetColors};
use crossterm::{cursor, ExecutableCommand};
use std::io::Write;
use std::time::{Duration, Instant};

/// A canvas implementation that renders to the terminal using crossterm.
///
/// This canvas provides terminal-based rendering using Unicode half blocks
/// and ANSI colors. It supports frame rate limiting and efficient updates
/// by only redrawing changed parts of the screen.
///
/// # Example
/// ```
/// use pixel_loop::canvas::CrosstermCanvas;
/// use pixel_loop::color::*;
/// use pixel_loop::canvas::Canvas;
/// use pixel_loop::canvas::RenderableCanvas;
/// use std::ops::Range;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///   let mut canvas = CrosstermCanvas::new(80, 24);
///   canvas.filled_rect(5, 5, 10, 10, &Color::from_rgb(255, 0, 0));
///   // Should of course be called within the [pixel_loop::run] function.
///   canvas.render()?;
///  Ok(())
/// }
/// ```
pub struct CrosstermCanvas {
    /// Width of the canvas in pixels (characters)
    width: u32,
    /// Height of the canvas in pixels (half characters)
    height: u32,
    /// Resizability of the canvas
    resizable: bool,
    /// Current frame buffer
    buffer: Vec<Color>,
    /// Previous frame buffer for change detection
    previous_buffer: Vec<Color>,
    /// Minimal frame time in nanoseconds
    frame_limit_nanos: u64,
    /// Timestamp of the last rendered frame
    last_frame_time: Instant,
    /// The width of this canvas during the last loop
    last_loop_width: u32,
    /// The height of this canvas during the last loop
    last_loop_height: u32,
}

impl CrosstermCanvas {
    /// Creates a new terminal canvas automatically toking the size of the
    /// terminal it is spawned in.
    ///
    /// A canvas based on the terminals size is resizable by default.
    ///
    /// # Example
    /// ```
    /// use pixel_loop::canvas::CrosstermCanvas;
    ///
    /// let canvas = CrosstermCanvas::new();
    /// ```
    pub fn new() -> Self {
        let (columns, rows) = crossterm::terminal::size().unwrap();
        Self::new_with_size(columns as u32, rows as u32 * 2).with_resizable(true)
    }

    /// Creates a new terminal canvas with the specified dimensions.
    ///
    /// A canvas with specified dimensions is not resizable by default.
    ///
    /// # Arguments
    /// * `width` - The width of the canvas in characters
    /// * `height` - The height of the canvas in half characters
    ///
    /// # Example
    /// ```
    /// use pixel_loop::canvas::CrosstermCanvas;
    ///
    /// let canvas = CrosstermCanvas::new(80, 42);
    /// ```
    pub fn new_with_size(width: u32, height: u32) -> Self {
        let mut canvas = Self {
            width,
            height,
            resizable: false,
            buffer: vec![],
            previous_buffer: vec![],
            frame_limit_nanos: 1_000_000_000 / 60,
            last_frame_time: Instant::now(),
            last_loop_height: 0, // Zero initialized to cause initial update
            last_loop_width: 0,  // Zero initialized to cause initial update
        };
        canvas.resize_surface(width, height, None);
        canvas
    }

    /// Sets the canvas to be resizable or not.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Sets the frame rate limit.
    ///
    /// # Arguments
    /// * `limit` - Target frames per second
    ///
    /// By default, the canvas is limited to 60 frames per second.
    ///
    /// # Example
    /// ```
    /// use pixel_loop::canvas::CrosstermCanvas;
    ///
    /// // Limit the frame rate to 30 frames per second
    /// let mut canvas = CrosstermCanvas::new(80, 24).with_refresh_limit(30);
    /// ```
    pub fn with_refresh_limit(mut self, limit: usize) -> Self {
        self.frame_limit_nanos = 1_000_000_000u64 / limit as u64;
        self
    }
}

impl Canvas for CrosstermCanvas {
    fn width(&self) -> u32 {
        self.width as u32
    }

    fn height(&self) -> u32 {
        self.height as u32
    }

    fn set_range(&mut self, range: std::ops::Range<usize>, color: &[Color]) {
        self.buffer[range].copy_from_slice(color);
    }

    fn get_range(&self, range: std::ops::Range<usize>) -> &[Color] {
        &self.buffer[range]
    }
}

/// Unicode character representing the upper half block used for drawing half
/// character height (quadratic) pixels.
const UNICODE_UPPER_HALF_BLOCK: &str = "▀";

/// Represents a region of the screen that needs to be updated.
///
/// A patch contains the position and color data for a sequence of
/// changed characters that can be efficiently written to the terminal.
struct Patch {
    /// Terminal position (x, y)
    position: (u16, u16),
    /// Raw ANSI data to be written
    data: Vec<u8>,
    /// Previous colors for change detection
    previous_colors: Option<(Color, Color)>,
}

impl Patch {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            position: (x, y),
            data: Vec::new(),
            previous_colors: None,
        }
    }

    pub fn apply<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.execute(cursor::MoveTo(self.position.0, self.position.1))?;
        writer.write_all(&self.data)?;
        Ok(())
    }

    pub fn add_two_row_pixel(&mut self, upper: &Color, lower: &Color) -> Result<()> {
        if self.previous_colors.is_none()
            || self.previous_colors.as_ref().unwrap() != &(*upper, *lower)
        {
            self.data.execute(SetColors(style::Colors::new(
                style::Color::Rgb {
                    r: upper.r,
                    g: upper.g,
                    b: upper.b,
                },
                style::Color::Rgb {
                    r: lower.r,
                    g: lower.g,
                    b: lower.b,
                },
            )))?;
            self.previous_colors = Some((*upper, *lower));
        }
        self.data.execute(Print(UNICODE_UPPER_HALF_BLOCK))?;
        Ok(())
    }
}

impl CrosstermCanvas {
    fn calculate_patches(&self) -> Result<Vec<Patch>> {
        let mut patches = Vec::new();
        let mut active_patch: Option<Patch> = None;

        for y in (0..self.height as usize).step_by(2) {
            for x in 0..self.width as usize {
                let y1 = self.buffer[y * self.width as usize + x];
                let y2 = if self.height % 2 != 0 {
                    Color::from_rgb(0, 0, 0)
                } else {
                    self.buffer[(y + 1) * self.width as usize + x]
                };

                let py1 = self.previous_buffer[y * self.width as usize + x];
                let py2 = if self.height % 2 != 0 {
                    Color::from_rgb(0, 0, 0)
                } else {
                    self.previous_buffer[(y + 1) * self.width as usize + x]
                };

                if y1 != py1 || y2 != py2 {
                    if active_patch.is_none() {
                        active_patch = Some(Patch::new(x as u16, (y / 2) as u16));
                    }

                    let patch = active_patch.as_mut().unwrap();
                    patch.add_two_row_pixel(&y1, &y2)?;
                } else if active_patch.is_some() {
                    patches.push(active_patch.take().unwrap());
                }
            }
            if active_patch.is_some() {
                patches.push(active_patch.take().unwrap());
            }
        }

        if active_patch.is_some() {
            patches.push(active_patch.take().unwrap());
        }

        Ok(patches)
    }

    fn elapsed_since_last_frame(&self) -> u64 {
        // The return value of as_nanos is a u128, but a Duration from_nanos is
        // created with a u64. We are therefore casting this value into a u64 or
        // use the frame_limit_nanos as a default. Because if we are out of
        // limits (which shouldn't really happen), we need to directly rerender
        // anyways.
        self.last_frame_time
            .elapsed()
            .as_nanos()
            .try_into()
            .unwrap_or(self.frame_limit_nanos)
    }

    fn wait_for_next_frame(&mut self) {
        fn wait_half_using_thread_sleep(elapsed_nanos: u64, frame_limit_nanos: u64) {
            let minimum_thread_sleep_nanos = 4_000_000;
            if elapsed_nanos < frame_limit_nanos
                && (frame_limit_nanos - elapsed_nanos) / 2 > minimum_thread_sleep_nanos
            {
                std::thread::sleep(Duration::from_nanos(
                    (frame_limit_nanos - elapsed_nanos) / 2,
                ));
            }
        }
        fn wait_using_spinlock(elapsed_nanos: u64, frame_limit_nanos: u64) {
            if elapsed_nanos < frame_limit_nanos {
                let wait_time = frame_limit_nanos - elapsed_nanos;
                let target_time = Instant::now() + Duration::from_nanos(wait_time);
                while Instant::now() < target_time {
                    std::hint::spin_loop();
                }
            }
        }
        // Sleep the thread for have of the wait time needed.
        // Unfortunately sleeping the frame is quite impercise, therefore we
        // can't wait exactly the needed amount of time. We only wait for 1/2
        // of the time using a thread sleep.
        wait_half_using_thread_sleep(self.elapsed_since_last_frame(), self.frame_limit_nanos);
        // The rest of the time we precisely wait using a spinlock
        wait_using_spinlock(self.elapsed_since_last_frame(), self.frame_limit_nanos);

        self.last_frame_time = Instant::now();
    }
}

impl RenderableCanvas for CrosstermCanvas {
    type Input = CrosstermInputState;

    fn render(&mut self) -> anyhow::Result<()> {
        self.wait_for_next_frame();

        let mut stdout = std::io::stdout();
        let mut buffer = Vec::new();

        buffer.execute(cursor::Hide)?;
        let patches = self.calculate_patches()?;
        for patch in patches {
            patch.apply(&mut buffer)?;
        }
        buffer.execute(cursor::MoveTo(
            self.width.try_into()?,
            (self.height / 2).try_into()?,
        ))?;
        buffer.execute(cursor::Show)?;
        stdout.write_all(&buffer)?;
        stdout.flush()?;

        self.previous_buffer.copy_from_slice(&self.buffer);

        Ok(())
    }

    fn resize_surface(&mut self, width: u32, height: u32, scale_factor: Option<f64>) {
        self.width = width;
        self.height = height;
        self.buffer = vec![Color::from_rgb(0, 0, 0); width as usize * height as usize];
        self.previous_buffer = vec![Color::from_rgba(0, 0, 0, 0); width as usize * height as usize];
    }

    /// Runs the pixel loop.
    fn run<State: 'static>(mut pixel_loop: crate::PixelLoop<State, Self>) -> ! {
        fn get_all_next_crossterm_events() -> Result<Vec<Event>> {
            use crossterm::event::{poll, read};
            let mut events = vec![];
            loop {
                if poll(Duration::from_secs(0))? {
                    let event = read()?;
                    events.push(event);
                } else {
                    break;
                }
            }

            Ok(events)
        }

        pixel_loop.begin().expect("begin pixel_loop");
        loop {
            for event in get_all_next_crossterm_events().expect("get_all_next_crossterm_events") {
                // Handle resizeing of the terminal
                if let Event::Resize(columns, rows) = event {
                    pixel_loop
                        .canvas
                        .resize_surface(columns as u32, rows as u32 * 2, None);
                }

                // Move elements to input state handler
                pixel_loop.input_state.handle_new_event(event);
            }

            let next = pixel_loop.next_loop().expect("next_loop pixel_loop");
            if let crate::NextLoopState::Exit(code) = next {
                pixel_loop.finish(code).expect("finish pixel loop");
            }
            // Track last communicated canvas size
            pixel_loop.canvas.last_loop_width = pixel_loop.canvas.width();
            pixel_loop.canvas.last_loop_height = pixel_loop.canvas.height();
        }
    }

    fn did_resize(&self) -> Option<(u32, u32)> {
        if self.last_loop_width != self.width() || self.last_loop_height != self.height() {
            Some((self.width(), self.height()))
        } else {
            None
        }
    }

    fn begin(&mut self) -> Result<()> {
        std::io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
        Ok(())
    }

    fn finish(&mut self, _code: i32) -> Result<()> {
        std::io::stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;
        Ok(())
    }
}
