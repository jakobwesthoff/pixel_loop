use std::time::SystemTime;

use anyhow::Result;
use crossterm::terminal;
use pixel_loop::crossterm_canvas::CrosstermCanvas;
use pixel_loop::{Canvas, Color, RenderableCanvas};

struct State {
    box_position: (i64, i64),
    box_direction: (i64, i64),
    box_size: (u32, u32),
    frame_count: usize,
    start_frame_time: SystemTime,
}

impl State {
    fn new(width: u32, height: u32) -> Self {
        Self {
            box_position: Default::default(),
            box_direction: (1, 1),
            box_size: (20, 10),
            frame_count: 0,
            start_frame_time: SystemTime::now(),
        }
    }
}

fn main() -> Result<()> {
    let (terminal_width, terminal_height) = terminal::size()?;
    let width = terminal_width;
    let height = terminal_height * 2;

    let canvas = CrosstermCanvas::new(width, height);

    let state = State::new(width as u32, height as u32);

    eprintln!("Render size: {width}x{height}");

    pixel_loop::run(
        60,
        state,
        canvas,
        |e, s, canvas| {
            let width = canvas.width();
            let height = canvas.height();

            let (mut px, mut py) = s.box_position;
            let (mut dx, mut dy) = s.box_direction;
            let (sx, sy) = s.box_size;
            px += dx;
            py += dy;

            if px < 0 || px + sx as i64 >= width as i64 {
                dx *= -1;
                px += dx;
            }
            if py < 0 || py + sy as i64 >= height as i64 {
                dy *= -1;
                py += dy;
            }

            s.box_position = (px, py);
            s.box_direction = (dx, dy);

            Ok(())
        },
        |e, s, canvas, dt| {
            if cfg!(feature = "benchmark_fps") {
                s.frame_count += 1;
            }

            let yellow = Color::from_rgb(255, 255, 128);
            let dark_yellow = Color::from_rgb(128, 128, 64);

            // RENDER BEGIN
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));

            canvas.filled_rect(
                s.box_position.0 + 2,
                s.box_position.1 + 2,
                s.box_size.0,
                s.box_size.1,
                &dark_yellow,
            );
            canvas.filled_rect(
                s.box_position.0,
                s.box_position.1,
                s.box_size.0,
                s.box_size.1,
                &yellow,
            );
            // RENDER END

            canvas.render()?;

            if cfg!(feature = "benchmark_fps") {
                let duration = s.start_frame_time.elapsed().unwrap();
                let secs = duration.as_secs();

                if secs >= 5 {
                    let nanos = duration.as_nanos();
                    let fps = 1_000_000_000f64 / (nanos / s.frame_count as u128) as f64;
                    eprintln!(
                        "Rendered {frame_count} frames in {secs_fraction}s resulting in {fps} fps.",
                        frame_count = s.frame_count,
                        secs_fraction =
                            secs as f64 + duration.subsec_micros() as f64 / 1_000_000f64
                    );
                    std::process::exit(0);
                }
            }
            Ok(())
        },
    )?;
    Ok(())
}