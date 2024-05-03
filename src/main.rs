use anyhow::{Context, Result};
use std::time::Duration;

mod pixel_loop;

struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,
    box_position: (isize, isize),
    box_direction: (isize, isize),
    box_size: (usize, usize),
}

impl Default for State {
    fn default() -> Self {
        Self {
            updates_called: Default::default(),
            renders_called: Default::default(),
            time_passed: Default::default(),
            box_position: Default::default(),
            box_direction: (2, 2),
            box_size: (50, 50),
        }
    }
}

fn main() -> Result<()> {
    let width = 640;
    let height = 480;

    let context =
        pixel_loop::init_tao_window("pixel loop", width, height).context("create tao window")?;
    let surface =
        pixel_loop::init_pixels(&context, width, height).context("initialize pixel surface")?;

    let state = State::default();
    pixel_loop::run_with_tao_and_pixels(
        state,
        context,
        surface,
        |s, surface| {
            s.box_position.0 = s.box_position.0 + s.box_direction.0;
            s.box_position.1 = s.box_position.1 + s.box_direction.1;
            if s.box_position.0 + s.box_size.0 as isize >= surface.width() as isize
                || s.box_position.0 < 0
            {
                s.box_direction.0 = s.box_direction.0 * -1;
                s.box_position.0 = s.box_position.0 + s.box_direction.0
            }
            if s.box_position.1 + s.box_size.1 as isize >= surface.height() as isize
                || s.box_position.1 < 0
            {
                s.box_direction.1 = s.box_direction.1 * -1;
                s.box_position.1 = s.box_position.1 + s.box_direction.1
            }

            s.updates_called += 1;
            Ok(())
            // println!("update");
        },
        |s, surface, dt| {
            let width = surface.width();
            let height = surface.height();
            let buf = surface.frame_mut();

            // Clear background
            for y in 0..height {
                for x in 0..width {
                    let i = ((y * width + x) * 4) as usize;
                    buf[i + 0] = 0;
                    buf[i + 1] = 0;
                    buf[i + 2] = 0;
                    buf[i + 3] = 255;
                }
            }

            for y in s.box_position.1 as usize..s.box_position.1 as usize + s.box_size.1 {
                for x in s.box_position.0 as usize..s.box_position.0 as usize + s.box_size.0 {
                    let i = ((y * width as usize + x) * 4) as usize;
                    buf[i + 0] = 255;
                    buf[i + 1] = 255;
                    buf[i + 2] = 0;
                    buf[i + 3] = 255;
                }
            }

            s.renders_called += 1;
            s.time_passed += dt;
            if s.time_passed > Duration::from_secs(1) {
                println!("Update FPS: {:.2}", s.updates_called as f64 / 1f64);
                println!("Render FPS: {:.2}", s.renders_called as f64 / 1f64);
                s.updates_called = 0;
                s.renders_called = 0;
                s.time_passed = Duration::default();
            }

            surface.render()?;

            Ok(())
        },
    );
}
