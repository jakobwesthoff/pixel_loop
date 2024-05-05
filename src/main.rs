use anyhow::{Context, Result};
use pixel_loop::{Color, Canvas};
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
    cursor_position: (u32, u32),
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
            s.updates_called += 1;
            // UPDATE BEGIN
            //
            // UPDATE END
            Ok(())
        },
        |s, surface, dt| {
            let width = surface.width();
            let height = surface.height();

            // RENDER BEGIN
            surface.clear_screen(&Color::from_rgb(255, 0, 0));
            surface.filled_rect(40, 40, 100, 100, &Color::from_rgb(255, 255, 0));
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

            surface.blit()?;

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
                        let pixel_position = surface
                            .physical_pos_to_surface_pos(position.x, position.y)
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
