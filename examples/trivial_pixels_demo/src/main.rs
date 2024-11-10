use anyhow::Result;
use pixel_loop::canvas::{Canvas, PixelsCanvas, RenderableCanvas};
use pixel_loop::color::Color;
use pixel_loop::input::{KeyboardKey, KeyboardState, PixelsInputState};
use pixel_loop::rand::Rng;
use pixel_loop::NextLoopState;

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
    flying_box: FlyingBox,
}

impl State {
    fn new() -> Self {
        Self {
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

    let canvas = PixelsCanvas::new(width, height, Some(2), "pixel_loop", true)?;
    let input = PixelsInputState::new();
    let state = State::new();

    pixel_loop::run(
        120,
        state,
        input,
        canvas,
        |e, s, i, canvas| {
            if i.is_key_pressed(KeyboardKey::Space) {
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

            Ok(NextLoopState::Continue)
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

            Ok(NextLoopState::Continue)
        },
    );
}
