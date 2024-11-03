use crate::color::Color;
use anyhow::Result;
use crossterm::style::{self, Print, SetColors};
use crossterm::{cursor, ExecutableCommand};
use std::io::Write;
use std::time::{Duration, Instant};

use super::{Canvas, RenderableCanvas};

pub struct CrosstermCanvas {
    width: u16,
    height: u16,
    buffer: Vec<Color>,
    previous_buffer: Vec<Color>,
    frame_limit_nanos: u64,
    last_frame_time: Instant,
}

impl CrosstermCanvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::from_rgb(0, 0, 0); width as usize * height as usize],
            previous_buffer: vec![Color::from_rgba(0, 0, 0, 0); width as usize * height as usize],
            frame_limit_nanos: 1_000_000_000 / 60,
            last_frame_time: Instant::now(),
        }
    }

    pub fn set_refresh_limit(&mut self, limit: usize) {
        self.frame_limit_nanos = 1_000_000_000u64 / limit as u64;
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

const UNICODE_UPPER_HALF_BLOCK: &'static str = "▀";

struct Patch {
    position: (u16, u16),
    data: Vec<u8>,
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
                } else {
                    if active_patch.is_some() {
                        patches.push(active_patch.take().unwrap());
                    }
                }
            }
            if active_patch.is_some() {
                patches.push(active_patch.take().unwrap());
            }
        }

        if active_patch.is_some() {
            patches.push(active_patch.take().unwrap());
        }

        return Ok(patches);
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
    fn render(&mut self) -> anyhow::Result<()> {
        self.wait_for_next_frame();

        let mut stdout = std::io::stdout();
        let mut buffer = Vec::new();

        buffer.execute(cursor::Hide)?;
        let patches = self.calculate_patches()?;
        for patch in patches {
            patch.apply(&mut buffer)?;
        }
        buffer.execute(cursor::MoveTo(self.width, self.height / 2))?;
        buffer.execute(cursor::Show)?;
        stdout.write_all(&buffer)?;
        stdout.flush()?;

        self.previous_buffer.copy_from_slice(&self.buffer);

        Ok(())
    }

    fn physical_pos_to_canvas_pos(&self, x: f64, y: f64) -> Option<(u32, u32)> {
        todo!()
    }

    fn resize_surface(&mut self, width: u32, height: u32) {
        todo!()
    }

    fn resize(&mut self, width: u32, height: u32) {
        todo!()
    }
}
