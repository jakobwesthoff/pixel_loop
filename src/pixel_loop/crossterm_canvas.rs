use crate::{Canvas, Color, RenderableCanvas};
use anyhow::Result;
use crossterm::style::{self, Print, ResetColor, SetColors};
use crossterm::{cursor, ExecutableCommand, QueueableCommand};
use std::io::Write;

pub struct CrosstermCanvas {
    width: u16,
    height: u16,
    buffer: Vec<Color>,
    previous_buffer: Vec<Color>,
}

impl CrosstermCanvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::from_rgb(0, 0, 0); width as usize * height as usize],
            previous_buffer: vec![Color::from_rgba(0, 0, 0, 0); width as usize * height as usize],
        }
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
        writer.queue(cursor::MoveTo(self.position.0, self.position.1))?;
        writer.queue(Print(std::str::from_utf8(&self.data)?))?;
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
}

impl RenderableCanvas for CrosstermCanvas {
    fn render(&mut self) -> anyhow::Result<()> {
        let mut stdout = std::io::stdout();

        stdout.queue(cursor::Hide)?;
        let patches = self.calculate_patches()?;
        for patch in patches {
            patch.apply(&mut stdout)?;
        }
        stdout.queue(cursor::MoveTo(self.width, self.height / 2))?;
        stdout.queue(cursor::Show)?;
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
