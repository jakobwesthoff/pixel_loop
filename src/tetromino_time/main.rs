use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use pixel_loop::{Canvas, Color, EngineEnvironment, RenderableCanvas};
use tetromino::{AnimStep, Tetromino};

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

    fn update(&mut self, ee: &mut EngineEnvironment, i: u32, char_width: u32) -> bool {
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
                        (i * char_width) as u32,
                        200,
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
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,
    digits: Vec<Digit>,
    last_change: SystemTime,
    last_time_digits: Vec<u8>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            updates_called: Default::default(),
            renders_called: Default::default(),
            time_passed: Default::default(),
            digits: Default::default(),
            last_change: SystemTime::UNIX_EPOCH,
            last_time_digits: Default::default(),
        }
    }
}

fn system_time_to_digits(time: &SystemTime) -> Vec<u8> {
    let dt: DateTime<Local> = DateTime::from(*time);
    dt.format("%H%M%S")
        .to_string()
        .chars()
        .map(|c| {
            c.to_string()
                .parse::<u8>()
                .expect("system time should always be parsable")
        })
        .collect::<Vec<u8>>()
}

fn main() -> Result<()> {
    let width = 50;
    let height = 50;
    let scale = 1;

    let context =
        pixel_loop::init_tao_window("tetromino_time", width * scale, height * scale, true)
            .context("create tao window")?;
    let canvas =
        pixel_loop::init_pixels(&context, width, height).context("initialize pixel canvas")?;

    let state = State::default();

    pixel_loop::run_with_tao_and_pixels(
        state,
        context,
        canvas,
        |ee, s, canvas| {
            s.updates_called += 1;

            // @TODO: take this somehow from the base block size or move the
            // spacing to the "font" somehow
            let char_width = 7 * 16;

            // UPDATE BEGIN
            for i in 0..s.digits.len() {
                if s.digits[i].update(ee, i as u32, char_width) {
                    s.last_change = SystemTime::now();
                }
            }

            if SystemTime::now()
                .duration_since(s.last_change)
                .expect("time to be going forward")
                > Duration::from_secs(1)
            {
                let now_digits = system_time_to_digits(&SystemTime::now());
                if s.last_time_digits.len() < 6 {
                    // No last time stored
                    s.digits = now_digits
                        .iter()
                        .map(|d| Digit::from_digit(*d))
                        .collect::<Vec<Digit>>();
                } else {
                    for i in 0..s.last_time_digits.len() {
                        if s.last_time_digits[i] != now_digits[i] {
                            s.digits[i] = Digit::from_digit(now_digits[i]);
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
                    tetromino.draw(canvas);
                }
            }
            for candidate in &s.digits {
                if let Some(tetromino) = &candidate.active {
                    tetromino.draw(canvas);
                }
            }
            // RENDER END

            s.renders_called += 1;
            s.time_passed += dt;
            if s.time_passed > Duration::from_secs(1) {
                println!("Update FPS: {:.2}", s.updates_called as f64 / 1f64);
                println!("Render FPS: {:.2}", s.renders_called as f64 / 1f64);
                s.updates_called = 0;
                s.renders_called = 0;
                s.time_passed = Duration::default();
            }

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
                }
                _ => {}
            }
            Ok(())
        },
    );
}
