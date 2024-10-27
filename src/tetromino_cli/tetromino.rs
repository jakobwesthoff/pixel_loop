use pixel_loop::{Canvas, Color};


#[derive(Debug)]
pub enum Shape {
    L,
    Square,
    T,
    Straight,
    Skew,
}

struct Tetromino {
    shape: Shape,
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
        Shape::L => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
        }
        Shape::Square => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
        }
        Shape::T => canvas.maybe_get(*x, *y + 1) != Some(&empty),
        Shape::Straight => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 2, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 3, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 4, *y + 1) != Some(&empty)
        }
        Shape::Skew => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
        }
        _ => panic!(
            "Collision calculation for {:?} shape not implemented yet",
            shape
        ),
    }
}

pub struct Board {
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

    pub fn add_tetromino(&mut self, x: i64, y: i64, color: Color, shape: Shape) {
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
                Shape::L => {
                    canvas.filled_rect(*x, *y - 2, 1, 3, color);
                    canvas.filled_rect(*x + 1, *y, 1, 1, color);
                }
                Shape::Square => {
                    canvas.filled_rect(*x, *y - 1, 2, 2, color);
                }
                Shape::T => {
                    canvas.filled_rect(*x - 1, *y - 1, 3, 1, color);
                    canvas.filled_rect(*x, *y, 1, 1, color);
                }
                Shape::Straight => {
                    canvas.filled_rect(*x, *y, 5, 1, color);
                }
                Shape::Skew => {
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
