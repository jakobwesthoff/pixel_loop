mod tetromino;

use anyhow::Result;
use crossterm::terminal;
use pixel_loop::canvas::CrosstermCanvas;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
use pixel_loop::{Canvas, Color, RenderableCanvas};
use rand::Rng;
use tetromino::{Board, Rotation};

struct State {
    board: Board,
}

impl State {
    fn new(width: u32, height: u32) -> Self {
        Self {
            board: Board::new(),
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
        30,
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
                let x = e.rand.gen_range(0..width as i64 - 1);
                let color =
                    Color::from_rgb(e.rand.gen::<u8>(), e.rand.gen::<u8>(), e.rand.gen::<u8>());
                let shape = match e.rand.gen_range(0..5) {
                    0 => tetromino::Shape::L,
                    1 => tetromino::Shape::Square,
                    2 => tetromino::Shape::Straight,
                    3 => tetromino::Shape::T,
                    4 => tetromino::Shape::Skew,
                    _ => panic!("Something very strange happend"),
                };

                // @FIXME: Only for testing, remove later
                s.board
                    .add_tetromino(x, 0, color, shape, Rotation::NoRotation);
            }

            s.board.update(canvas);

            Ok(())
        },
        |e, s, i, canvas, dt| {
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));

            s.board.render(canvas);

            canvas.render()?;

            Ok(())
        },
    )?;
    Ok(())
}
