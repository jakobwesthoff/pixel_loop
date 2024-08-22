use crate::{Canvas, Color, RenderableCanvas};
use crossterm::style::{self, Print, ResetColor, SetColors};
use crossterm::{cursor, QueueableCommand};
use std::io::Write;

pub struct CrosstermCanvas {
    width: u16,
    height: u16,
    buffer: Vec<Color>,
}

impl CrosstermCanvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::from_rgb(0, 0, 0); width as usize * height as usize],
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

impl RenderableCanvas for CrosstermCanvas {
    fn render(&mut self) -> anyhow::Result<()> {
        let mut stdout = std::io::stdout();

        stdout.queue(cursor::Hide)?;
        stdout.queue(cursor::MoveTo(0, 0))?;
        for y in (0..self.height as usize).step_by(2) {
            for x in 0..self.width as usize {
                let y1 = self.buffer[y * self.width as usize + x];
                let y2 = if self.height % 2 != 0 {
                    Color::from_rgb(0, 0, 0)
                } else {
                    self.buffer[(y + 1) * self.width as usize + x]
                };

                stdout.queue(SetColors(style::Colors::new(
                    style::Color::Rgb {
                        r: y1.r,
                        g: y1.g,
                        b: y1.b,
                    },
                    style::Color::Rgb {
                        r: y2.r,
                        g: y2.g,
                        b: y2.b,
                    },
                )))?;
                stdout.queue(Print(UNICODE_UPPER_HALF_BLOCK))?;
            }
            stdout.queue(ResetColor)?;
            // Don't print newline after last line
            if y + 1 < self.height as usize - 1 {
                stdout.queue(Print("\n"))?;
            }
        }
        stdout.queue(cursor::Show)?;
        stdout.flush()?;

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
