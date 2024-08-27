use anyhow::Result;
use crossterm::terminal;
use pixel_loop::crossterm_canvas::CrosstermCanvas;
use pixel_loop::{Canvas, Color, RenderableCanvas};

struct State {
    box_position: (i64, i64),
    box_direction: (i64, i64),
    box_size: (u32, u32),
}

impl State {
    fn new(width: u32, height: u32) -> Self {
        Self {
            box_position: Default::default(),
            box_direction: (1, 1),
            box_size: (20, 10),
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
            Ok(())
        },
    )?;
    Ok(())
}
