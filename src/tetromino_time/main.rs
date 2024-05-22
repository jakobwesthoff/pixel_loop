use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use pixel_loop::{Canvas, Color, EngineEnvironment, RenderableCanvas};
use tetromino::{AnimStep, Tetromino, BLOCK_SIZE, DIGIT_HEIGHT, DIGIT_WIDTH};

mod character_animations;
mod number_animations;
mod tetromino;

#[derive(Default)]
struct Digit {
    anim_queue: VecDeque<AnimStep>,
    active: Option<Tetromino>,
    fallen: Vec<Tetromino>,
}

impl Digit {
    fn from_digit(digit: u8) -> Self {
        Self {
            anim_queue: number_animations::from_digit(digit).to_vec().into(),
            active: None,
            fallen: vec![],
        }
    }

    fn seperator() -> Self {
        Self {
            anim_queue: character_animations::COLON.to_vec().into(),
            active: None,
            fallen: vec![],
        }
    }

    fn update(&mut self, ee: &mut EngineEnvironment, i: u32, digits_offset: &(i64, i64)) -> bool {
        match self.active {
            Some(ref mut tetromino) => {
                tetromino.update(&mut ee.rand);
                if tetromino.is_finished() {
                    let tetromino = self.active.take().unwrap();
                    self.fallen.push(tetromino);
                }
                true
            }
            None => {
                if let Some(next_step) = self.anim_queue.pop_front() {
                    self.active = Some(Tetromino::from_anim_step(
                        next_step,
                        &mut ee.rand,
                        (i * (DIGIT_WIDTH + BLOCK_SIZE)) as u32,
                        -digits_offset.1,
                    ));
                    true
                } else {
                    false
                }
            }
        }
    }
}

struct State {
    digits: Vec<Digit>,
    last_change: SystemTime,
    last_time_digits: Vec<TimeElement>,
    digits_offset: (i64, i64),
    digits_size: (u32, u32),
}

impl Default for State {
    fn default() -> Self {
        Self {
            digits: Default::default(),
            last_change: SystemTime::UNIX_EPOCH,
            last_time_digits: Default::default(),
            digits_offset: (0, 0),
            digits_size: (DIGIT_WIDTH * 8 + BLOCK_SIZE * 5, DIGIT_HEIGHT),
        }
    }
}

#[derive(PartialEq)]
enum TimeElement {
    Digit(u8),
    Seperator,
}

fn system_time_to_time_elements(time: &SystemTime) -> Vec<TimeElement> {
    let dt: DateTime<Local> = DateTime::from(*time);
    dt.format("%H:%M:%S")
        .to_string()
        .chars()
        .map(|c| {
            use TimeElement::*;
            let result = c.to_string().parse::<u8>();
            match result {
                Ok(d) => Digit(d),
                Err(_) => Seperator,
            }
        })
        .collect::<Vec<TimeElement>>()
}

fn main() -> Result<()> {
    let state = State::default();

    let context = pixel_loop::init_tao_window(
        "tetromino_time",
        state.digits_size.0,
        state.digits_size.1,
        true,
    )
    .context("create tao window")?;
    let canvas = pixel_loop::init_pixels(&context, state.digits_size.0, state.digits_size.1)
        .context("initialize pixel canvas")?;

    pixel_loop::run_with_tao_and_pixels(
        state,
        context,
        canvas,
        |ee, s, canvas| {
            // @TODO: take this somehow from the base block size or move the
            // spacing to the "font" somehow
            let char_width = 7 * 16;

            // UPDATE BEGIN
            for i in 0..s.digits.len() {
                if s.digits[i].update(ee, i as u32, &s.digits_offset) {
                    s.last_change = SystemTime::now();
                }
            }

            if SystemTime::now()
                .duration_since(s.last_change)
                .expect("time to be going forward")
                > Duration::from_secs(1)
            {
                let now_digits = system_time_to_time_elements(&SystemTime::now());
                if s.last_time_digits.len() < 8 {
                    // No last time stored
                    s.digits = now_digits
                        .iter()
                        .map(|te| match te {
                            TimeElement::Digit(d) => Digit::from_digit(*d),
                            TimeElement::Seperator => Digit::seperator(),
                        })
                        .collect::<Vec<Digit>>();
                } else {
                    for i in 0..s.last_time_digits.len() {
                        if s.last_time_digits[i] != now_digits[i] {
                            s.digits[i] = match now_digits[i] {
                            TimeElement::Digit(d) => Digit::from_digit(d),
                            TimeElement::Seperator => Digit::seperator(),
                        }
                        }
                    }
                }
                s.last_time_digits = now_digits;
            }
            // UPDATE END

            Ok(())
        },
        |ee, s, canvas, dt| {
            let width = canvas.width();
            let height = canvas.height();

            // RENDER BEGIN
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));
            for digit in &s.digits {
                for tetromino in &digit.fallen {
                    tetromino.draw(canvas, s.digits_offset);
                }
            }
            for candidate in &s.digits {
                if let Some(tetromino) = &candidate.active {
                    tetromino.draw(canvas, s.digits_offset);
                }
            }
            // RENDER END

            canvas.render()?;

            Ok(())
        },
        |ee, s, canvas, window, event| {
            use tao::event::Event::*;
            use tao::event::WindowEvent::*;
            match event {
                WindowEvent {
                    event: Resized(new_size),
                    ..
                } => {
                    let logical_new_size = new_size.to_logical(window.scale_factor());
                    canvas.resize_surface(new_size.width, new_size.height);
                    canvas.resize(logical_new_size.width, logical_new_size.height);
                    // Center the digits
                    s.digits_offset = (
                        ((logical_new_size.width - s.digits_size.0) / 2) as i64,
                        ((logical_new_size.height - s.digits_size.1) / 2) as i64,
                    );
                }
                _ => {}
            }
            Ok(())
        },
    );
}
