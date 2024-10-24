use anyhow::Result;
use crossterm::terminal;
use pixel_loop::canvas::CrosstermCanvas;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
use pixel_loop::{Canvas, Color, RenderableCanvas};
use rand::Rng;

#[derive(Debug)]
enum TetrominoShape {
    L,
    Square,
    T,
    Straight,
    Skew,
}

struct Tetromino {
    shape: TetrominoShape,
    x: i64,
    y: i64,
    color: Color,
    stopped: bool,
}

fn would_tetromino_collide_with_canvas<C: Canvas>(
    Tetromino { shape, x, y, .. }: &Tetromino,
    canvas: &C,
) -> bool {
    let empty = Color::from_rgb(0, 0, 0);
    match shape {
        TetrominoShape::L => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
        }
        TetrominoShape::Square => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
        }
        TetrominoShape::T => canvas.maybe_get(*x, *y + 1) != Some(&empty),
        TetrominoShape::Straight => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 2, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 3, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 4, *y + 1) != Some(&empty)
        }
        TetrominoShape::Skew => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
        }
        _ => panic!(
            "Collision calculation for {:?} shape not implemented yet",
            shape
        ),
    }
}

struct Board {
    tetrominos: Vec<Tetromino>,
    virtual_y_stop: i64,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tetrominos: vec![],
            // @FIXME: Calculate based on terminal height and shown digits
            // height, to center display.
            virtual_y_stop: 40,
        }
    }

    pub fn add_tetromino(&mut self, x: i64, y: i64, color: Color, shape: TetrominoShape) {
        self.tetrominos.push(Tetromino {
            x,
            y,
            color,
            shape,
            stopped: false,
        })
    }

    pub fn render<C: Canvas>(&self, canvas: &mut C) {
        for Tetromino {
            shape, x, y, color, ..
        } in self.tetrominos.iter()
        {
            match shape {
                TetrominoShape::L => {
                    canvas.filled_rect(*x, *y - 2, 1, 3, color);
                    canvas.filled_rect(*x + 1, *y, 1, 1, color);
                }
                TetrominoShape::Square => {
                    canvas.filled_rect(*x, *y - 1, 2, 2, color);
                }
                TetrominoShape::T => {
                    canvas.filled_rect(*x - 1, *y - 1, 3, 1, color);
                    canvas.filled_rect(*x, *y, 1, 1, color);
                }
                TetrominoShape::Straight => {
                    canvas.filled_rect(*x, *y, 5, 1, color);
                }
                TetrominoShape::Skew => {
                    canvas.filled_rect(*x, *y, 2, 1, color);
                    canvas.filled_rect(*x + 1, *y - 1, 2, 1, color);
                }
                _ => panic!(
                    "Render implementation for {:?} shape not implemented yet",
                    shape
                ),
            }
        }
    }

    pub fn update<C: Canvas>(&mut self, canvas: &C) {
        for tetromino in self.tetrominos.iter_mut() {
            if !tetromino.stopped && !would_tetromino_collide_with_canvas(tetromino, canvas) {
                tetromino.y += 1;
            }

            if tetromino.y == self.virtual_y_stop {
                tetromino.stopped = true;
            }
        }
    }
}

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
                    0 => TetrominoShape::L,
                    1 => TetrominoShape::Square,
                    2 => TetrominoShape::Straight,
                    3 => TetrominoShape::T,
                    4 => TetrominoShape::Skew,
                    _ => panic!("Something very strange happend")
                };

                // @FIXME: Only for testing, remove later
                s.board.add_tetromino(x, 0, color, shape);
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
