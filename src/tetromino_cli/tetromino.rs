use pixel_loop::{Canvas, Color};

#[derive(Debug)]
pub enum Shape {
    L,
    Square,
    T,
    Straight,
    Skew,
}

#[derive(Debug)]
pub enum Rotation {
    Degrees90,
    Degrees180,
    Degrees270,
    NoRotation,
}

struct Tetromino {
    shape: Shape,
    rotation: Rotation,
    x: i64,
    y: i64,
    color: Color,
    stopped: bool,
}

fn would_tetromino_collide_with_canvas<C: Canvas>(
    Tetromino {
        shape,
        rotation,
        x,
        y,
        ..
    }: &Tetromino,
    canvas: &C,
) -> bool {
    let empty = Color::from_rgb(0, 0, 0);
    use Rotation::*;
    use Shape::*;
    match (shape, rotation) {
        (L, NoRotation) => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
        }
        (Square, _) => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
        }
        (T, NoRotation) => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y) != Some(&empty)
                || canvas.maybe_get(*x - 1, *y) != Some(&empty)
        }
        (Straight, NoRotation) => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 2, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 3, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 4, *y + 1) != Some(&empty)
        }
        (Skew, NoRotation) => {
            canvas.maybe_get(*x, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 1, *y + 1) != Some(&empty)
                || canvas.maybe_get(*x + 2, *y) != Some(&empty)
                || canvas.maybe_get(*x + 3, *y) != Some(&empty)
        }
        _ => panic!(
            "Collision calculation for {:?} shape and rotation {:?} not implemented yet",
            shape, rotation
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

    pub fn add_tetromino(
        &mut self,
        x: i64,
        y: i64,
        color: Color,
        shape: Shape,
        rotation: Rotation,
    ) {
        self.tetrominos.push(Tetromino {
            x,
            y,
            color,
            shape,
            rotation,
            stopped: false,
        })
    }

    pub fn render<C: Canvas>(&self, canvas: &mut C) {
        for Tetromino {
            shape,
            rotation,
            x,
            y,
            color,
            ..
        } in self.tetrominos.iter()
        {
            use Rotation::*;
            use Shape::*;
            match (shape, rotation) {
                (L, NoRotation) => {
                    canvas.filled_rect(*x, *y - 2, 1, 3, color);
                    canvas.filled_rect(*x + 1, *y, 1, 1, color);
                }
                (Square, _) => {
                    canvas.filled_rect(*x, *y - 1, 2, 2, color);
                }
                (T, NoRotation) => {
                    canvas.filled_rect(*x - 1, *y - 1, 3, 1, color);
                    canvas.filled_rect(*x, *y, 1, 1, color);
                }
                (Straight, NoRotation) => {
                    canvas.filled_rect(*x, *y, 5, 1, color);
                }
                (Skew, NoRotation) => {
                    canvas.filled_rect(*x, *y, 2, 1, color);
                    canvas.filled_rect(*x + 1, *y - 1, 2, 1, color);
                }
                _ => panic!(
                    "Render implementation for {:?} shape with rotation {:?} not implemented yet",
                    shape, rotation
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
