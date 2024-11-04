use anyhow::{Context, Result};
use pixel_loop::canvas::{Canvas, RenderableCanvas};
use pixel_loop::color::Color;
use pixel_loop::rand::Rng;
use pixel_loop::winit::winit::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent,
};

struct FlyingBox {
    x: i64,
    y: i64,
    width: u32,
    height: u32,
    speed_x: i64,
    speed_y: i64,
    color: Color,
}

struct State {
    space_is_pressed: bool,
    flying_box: FlyingBox,
}

impl State {
    fn new() -> Self {
        Self {
            space_is_pressed: false,
            flying_box: FlyingBox {
                x: 0,
                y: 0,
                width: 64,
                height: 64,
                speed_x: 2,
                speed_y: 2,
                color: Color::from_rgb(156, 80, 182),
            },
        }
    }
}

fn main() -> Result<()> {
    let width = 640;
    let height = 480;

    let context = pixel_loop::winit::init_window("pixel loop", width, height, false)
        .context("create winit window")?;
    let canvas = pixel_loop::winit::init_pixels(&context, width, height)
        .context("initialize pixel canvas")?;

    let state = State::new();

    pixel_loop::winit::run(
        120,
        state,
        // @TODO: Just a placeholder. Implement proper input state for winint
        // and use here!
        pixel_loop::input::NoopInputState::new(),
        context,
        canvas,
        |e, s, i, canvas| {
            // @TODO: Replace with proper input handling once implemented.
            if s.space_is_pressed {
                // Randomise color on press of space
                s.flying_box.color =
                    Color::from_rgb(e.rand.gen::<u8>(), e.rand.gen::<u8>(), e.rand.gen::<u8>());
            }

            s.flying_box.x += s.flying_box.speed_x;
            s.flying_box.y += s.flying_box.speed_y;
            if s.flying_box.x + s.flying_box.width as i64 >= canvas.width() as i64
                || s.flying_box.x <= 0
            {
                s.flying_box.speed_x *= -1;
                s.flying_box.x += s.flying_box.speed_x;
            }
            if s.flying_box.y + s.flying_box.height as i64 >= canvas.height() as i64
                || s.flying_box.y <= 0
            {
                s.flying_box.speed_y *= -1;
                s.flying_box.y += s.flying_box.speed_y;
            }

            // @TODO: Replace with proper input handling once implemented.
            s.space_is_pressed = false;
            Ok(())
        },
        |e, s, i, canvas, dt| {
            let width = canvas.width();
            let height = canvas.height();

            // RENDER BEGIN
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));
            canvas.filled_rect(
                s.flying_box.x,
                s.flying_box.y,
                s.flying_box.width,
                s.flying_box.height,
                &s.flying_box.color,
            );
            // RENDER END

            canvas.render()?;

            Ok(())
        },
        |e, s, canvas, _, input, event| {
            // @TODO: Remove completely and replace with proper input handling
            // once implemented.
            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(VirtualKeyCode::Space),
                            ..
                        } => {
                            s.space_is_pressed = true;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
            Ok(())
        },
    );
}
