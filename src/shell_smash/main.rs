use anyhow::Result;
use crossterm::terminal;
use pixel_loop::canvas::CrosstermCanvas;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
use pixel_loop::{Canvas, Color, RenderableCanvas};

struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}

fn main() -> Result<()> {
    let (terminal_width, terminal_height) = terminal::size()?;
    let width = terminal_width;
    let height = terminal_height * 2;

    let mut canvas = CrosstermCanvas::new(width, height);
    canvas.set_refresh_limit(120);

    let state = State::new();
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

            Ok(())
        },
        |e, s, i, canvas, dt| {
            // RENDER BEGIN
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));

            // RENDER END

            canvas.render()?;

            Ok(())
        },
    )?;
    Ok(())
}
