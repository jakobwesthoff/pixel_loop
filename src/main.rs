use anyhow::{Context, Result};
use std::time::Duration;
use tao::event::{ElementState, Event, MouseButton, WindowEvent};

mod pixel_loop;

struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,
    box_position: (isize, isize),
    box_direction: (isize, isize),
    box_size: (usize, usize),
    button_pressed: bool,
    cursor_position: (usize, usize),
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
            button_pressed: false,
            cursor_position: (0, 0),
        }
    }
}

fn main() -> Result<()> {
    let width = 640;
    let height = 480;
    let scale = 1;

    let context =
        pixel_loop::init_tao_window("pixel loop", width, height).context("create tao window")?;
    let surface = pixel_loop::init_pixels(&context, width / scale, height / scale)
        .context("initialize pixel surface")?;

    let state = State::default();

    pixel_loop::run_with_tao_and_pixels(
        state,
        context,
        surface,
        |s, surface| {
            s.updates_called += 1;
            // UPDATE BEGIN
            //
            // UPDATE END
            Ok(())
        },
        |s, surface, dt| {
            let width = surface.width();
            let height = surface.height();
            let buf = surface.frame_mut();

            // RENDER BEGIN
            for y in 0..height {
                for x in 0..width {
                    let i = ((y * width + x) * 4) as usize;
                    buf[i + 0] = 0;
                    buf[i + 1] = 0;
                    buf[i + 2] = 0;
                    buf[i + 3] = 0;
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

            surface.render()?;

            Ok(())
        },
        |s, surface, _, event| {
            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state,
                        ..
                    } => {
                        if state == &ElementState::Pressed {
                            s.button_pressed = true;
                        } else {
                            s.button_pressed = false;
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let position = (position.x as f32, position.y as f32);
                        let pixel_position = surface
                            .pixels()
                            .window_pos_to_pixel(position)
                            .unwrap_or((0, 0));
                        s.cursor_position = pixel_position;
                    }
                    _ => {}
                },
                _ => {}
            }
            Ok(())
        },
    );
}
