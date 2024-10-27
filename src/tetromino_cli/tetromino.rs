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

// Tetromino coordinates always describe the lower left corner of the shape,
// where it is filled.
// Exanmple:
// xxx
//  x
//  ^ Lower(!) left corner of this pixel is the coordinate of the tetromino.
//
// This is in contrast to the usual coordinate system where the upper left
// corner is used. Positioning that way, makes the resoning about laying
// out the tetrominos to form a clock easier in the end.
//
// This kind of "messes" up rotation, as there is no fixed "center" to rotate
// around. However as we are not in the business of implementing a tetris game
// this is not important to us. Rotationonal symetry is not a requirement for
// the clock.  The shapes are based upon this reference:
// https://tetris.wiki/images/b/b5/Tgm_basic_ars_description.png
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
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y - 1, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y - 1, &empty)
        }
        (L, Degrees90) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 2, &empty)
        }
        (L, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y, &empty)
        }
        (L, Degrees270) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
        }
        (Square, _) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
        }
        (T, NoRotation) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y - 1, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 1, &empty)
        }
        (T, Degrees90) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 1, &empty)
        }
        (T, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y, &empty)
        }
        (T, Degrees270) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y - 1, &empty)
        }
        (Straight, NoRotation) | (Straight, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y, &empty)
                || !canvas.is_empty_or_color(*x + 3, *y, &empty)
        }
        (Straight, Degrees90) | (Straight, Degrees270) => !canvas.is_empty_or_color(*x, *y, &empty),
        (Skew, NoRotation) | (Skew, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y - 1, &empty)
        }
        (Skew, Degrees90) | (Skew, Degrees270) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 1, &empty)
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
                    canvas.filled_rect(*x, *y - 2, 1, 2, color);
                    canvas.filled_rect(*x + 1, *y - 2, 2, 1, color);
                }
                (L, Degrees90) => {
                    canvas.filled_rect(*x, *y - 3, 1, 3, color);
                    canvas.filled_rect(*x - 1, *y - 3, 1, 1, color);
                }
                (L, Degrees180) => {
                    canvas.filled_rect(*x, *y - 1, 3, 1, color);
                    canvas.filled_rect(*x + 2, *y - 2, 1, 1, color);
                }
                (L, Degrees270) => {
                    canvas.filled_rect(*x, *y - 3, 1, 3, color);
                    canvas.filled_rect(*x + 1, *y - 1, 1, 1, color);
                }
                (Square, _) => {
                    canvas.filled_rect(*x, *y - 2, 2, 2, color);
                }
                (T, NoRotation) => {
                    canvas.filled_rect(*x - 1, *y - 2, 3, 1, color);
                    canvas.filled_rect(*x, *y - 1, 1, 1, color);
                }
                (T, Degrees90) => {
                    canvas.filled_rect(*x, *y - 3, 1, 3, color);
                    canvas.filled_rect(*x - 1, *y - 2, 1, 1, color);
                }
                (T, Degrees180) => {
                    canvas.filled_rect(*x, *y - 1, 3, 1, color);
                    canvas.filled_rect(*x + 1, *y - 2, 1, 1, color);
                }
                (T, Degrees270) => {
                    canvas.filled_rect(*x, *y - 3, 1, 3, color);
                    canvas.filled_rect(*x + 1, *y - 2, 1, 1, color);
                }
                (Straight, NoRotation) | (Straight, Degrees180) => {
                    canvas.filled_rect(*x, *y - 1, 4, 1, color);
                }
                (Straight, Degrees90) | (Straight, Degrees270) => {
                    canvas.filled_rect(*x, *y - 4, 1, 4, color);
                }
                (Skew, NoRotation) | (Skew, Degrees180) => {
                    canvas.filled_rect(*x, *y - 1, 2, 1, color);
                    canvas.filled_rect(*x + 1, *y - 2, 2, 1, color);
                }
                (Skew, Degrees90) | (Skew, Degrees270) => {
                    canvas.filled_rect(*x, *y - 2, 1, 2, color);
                    canvas.filled_rect(*x - 1, *y - 3, 1, 2, color);
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
