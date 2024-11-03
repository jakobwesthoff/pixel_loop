use anyhow::Result;
use pixel_loop::crossterm::terminal;
use pixel_loop::canvas::{Canvas, CrosstermCanvas, RenderableCanvas};
use pixel_loop::color::*;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
use pixel_loop::rand::Rng;

struct Box {
    box_position: (i64, i64),
    box_direction: (i64, i64),
    box_size: (u32, u32),
    color: Color,
    shadow_color: Color,
}

struct State {
    my_box: Box,
    boxes: Vec<Box>,
}

impl State {
    fn new(width: u32, height: u32) -> Self {
        Self {
            my_box: Box {
                box_position: (0, 0),
                box_direction: (1, 1),
                box_size: (5, 5),
                color: Color::from_rgb(156, 80, 182),
                shadow_color: Color::from_rgb(104, 71, 141),
            },
            boxes: vec![
                Box {
                    box_position: (0, 0),
                    box_direction: (1, 1),
                    box_size: (20, 10),
                    color: Color::from_rgb(255, 255, 128),
                    shadow_color: Color::from_rgb(128, 128, 64),
                },
                Box {
                    box_position: (0, 4),
                    box_direction: (2, 1),
                    box_size: (5, 5),
                    color: Color::from_rgb(128, 255, 128),
                    shadow_color: Color::from_rgb(64, 128, 64),
                },
                Box {
                    box_position: (0, 23),
                    box_direction: (1, 2),
                    box_size: (20, 20),
                    color: Color::from_rgb(255, 128, 64),
                    shadow_color: Color::from_rgb(128, 64, 32),
                },
                Box {
                    box_position: (0, 10),
                    box_direction: (2, 2),
                    box_size: (10, 10),
                    color: Color::from_rgb(255, 0, 128),
                    shadow_color: Color::from_rgb(128, 0, 64),
                },
            ],
        }
    }
}

fn main() -> Result<()> {
    let (terminal_width, terminal_height) = terminal::size()?;
    let width = terminal_width;
    let height = terminal_height * 2;

    let mut canvas = CrosstermCanvas::new(width, height);
    canvas.set_refresh_limit(120);

    let state = State::new(width as u32, height as u32);
    let input = CrosstermInputState::new();

    eprintln!("Render size: {width}x{height}");

    pixel_loop::run(
        60,
        state,
        input,
        canvas,
        |e, s, input, canvas| {
            let width = canvas.width();
            let height = canvas.height();

            if input.is_key_pressed(KeyboardKey::Q) {
                std::process::exit(0);
            }

            if input.is_key_pressed(KeyboardKey::Space) {
                for b in s.boxes.iter_mut() {
                    b.color = Color::from_rgb(e.rand.gen(), e.rand.gen(), e.rand.gen());
                    let mut shadow_color = b.color.as_hsl();
                    shadow_color.s = (shadow_color.s - 20.0).clamp(0.0, 100.0);
                    shadow_color.l = (shadow_color.l - 20.0).clamp(0.0, 100.0);
                    b.shadow_color = Color::from(shadow_color);
                }
            }

            if input.is_key_down(KeyboardKey::Up) {
                s.my_box.box_position.1 -= 1;
            }
            if input.is_key_down(KeyboardKey::Down) {
                s.my_box.box_position.1 += 1;
            }
            if input.is_key_down(KeyboardKey::Left) {
                s.my_box.box_position.0 -= 1;
            }
            if input.is_key_down(KeyboardKey::Right) {
                s.my_box.box_position.0 += 1;
            }

            for b in s.boxes.iter_mut() {
                let (mut px, mut py) = b.box_position;
                let (mut dx, mut dy) = b.box_direction;
                let (sx, sy) = b.box_size;
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

                b.box_position = (px, py);
                b.box_direction = (dx, dy);
            }

            Ok(())
        },
        |e, s, i, canvas, dt| {
            // RENDER BEGIN
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));

            for b in s.boxes.iter() {
                canvas.filled_rect(
                    b.box_position.0 + 2,
                    b.box_position.1 + 2,
                    b.box_size.0,
                    b.box_size.1,
                    &b.shadow_color,
                );
                canvas.filled_rect(
                    b.box_position.0,
                    b.box_position.1,
                    b.box_size.0,
                    b.box_size.1,
                    &b.color,
                );
            }
            canvas.filled_rect(
                s.my_box.box_position.0 + 1,
                s.my_box.box_position.1 + 1,
                s.my_box.box_size.0,
                s.my_box.box_size.1,
                &s.my_box.shadow_color,
            );
            canvas.filled_rect(
                s.my_box.box_position.0,
                s.my_box.box_position.1,
                s.my_box.box_size.0,
                s.my_box.box_size.1,
                &s.my_box.color,
            );

            // RENDER END

            canvas.render()?;

            Ok(())
        },
    )?;
    Ok(())
}
