use std::collections::VecDeque;
use std::time::Duration;

use anyhow::{Context, Result};
use pixel_loop::{Canvas, Color, RenderableCanvas};
use tetromino::{AnimStep, Tetromino};

mod number_animations;
mod tetromino;

#[derive(Default)]
struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,
    digits_anim: Vec<VecDeque<AnimStep>>,
    digits_active: Vec<Option<Tetromino>>,
    digits_fallen: Vec<Vec<Tetromino>>,
}

fn main() -> Result<()> {
    let width = 1280;
    let height = 720;
    let scale = 1;

    let context = pixel_loop::init_tao_window("tetromino_time", width * scale, height * scale)
        .context("create tao window")?;
    let canvas =
        pixel_loop::init_pixels(&context, width, height).context("initialize pixel canvas")?;

    let mut state = State::default();
    state.digits_anim = vec![
        VecDeque::from(number_animations::ZERO.to_vec()),
        VecDeque::from(number_animations::ONE.to_vec()),
        VecDeque::from(number_animations::TWO.to_vec()),
        VecDeque::from(number_animations::THREE.to_vec()),
        VecDeque::from(number_animations::FOUR.to_vec()),
        VecDeque::from(number_animations::FIVE.to_vec()),
        VecDeque::from(number_animations::SIX.to_vec()),
        VecDeque::from(number_animations::SEVEN.to_vec()),
        VecDeque::from(number_animations::EIGHT.to_vec()),
        VecDeque::from(number_animations::NINE.to_vec()),
    ];
    state.digits_active = vec![None, None, None, None, None, None, None, None, None, None];
    state.digits_fallen = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    pixel_loop::run_with_tao_and_pixels(
        state,
        context,
        canvas,
        |e, s, canvas| {
            // @TODO: take this somehow from the base block size or move the
            // spacing to the "font" somehow
            let char_width = 7 * 16;
            // UPDATE BEGIN

            for i in 0..s.digits_active.len() {
                match s.digits_active[i] {
                    Some(ref mut tetromino) => {
                        tetromino.update(&mut e.rand);
                        if tetromino.is_finished() {
                            let tetromino = s.digits_active[i].take().unwrap();
                            s.digits_fallen[i].push(tetromino);
                        }
                    }
                    None => {
                        if let Some(next_step) = s.digits_anim[i].pop_front() {
                            s.digits_active[i] = Some(Tetromino::from_anim_step(
                                next_step,
                                &mut e.rand,
                                (i * char_width) as u32,
                                200,
                            ));
                        }
                    }
                }
            }
            // UPDATE END

            Ok(())
        },
        |e, s, canvas, dt| {
            let width = canvas.width();
            let height = canvas.height();

            // RENDER BEGIN
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));
            for digit in &s.digits_fallen {
                for tetromino in digit {
                    tetromino.draw(canvas);
                }
            }
            for candidate in &s.digits_active {
                if let Some(tetromino) = candidate {
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
        |e, s, canvas, _, event| Ok(()),
    );
}