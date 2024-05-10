use pixel_loop::{Canvas, Color};

//   int blocktype;  // Number of the block type
//   int color; // Color of the brick
//   int x_pos;      // x-position (starting from the left number staring point) where the brick should be placed
//   int y_stop;     // y-position (1-16, where 16 is the last line of the matrix) where the brick should stop falling
//   int num_rot;    // Number of 90-degree (clockwise) rotations a brick is turned from the standard position
#[derive(Clone)]
pub struct AnimStep(pub u32, pub u32, pub u32, pub u32, pub u8);

impl AnimStep {
    fn as_color(&self) -> Color {
        match self.1 {
            // RED;
            0 => Color::from_rgb(255, 0, 0),
            // GREEN;
            1 => Color::from_rgb(0, 255, 0),
            // BLUE;
            2 => Color::from_rgb(0, 0, 255),
            // WHITE;
            3 => Color::from_rgb(255, 255, 255),
            // YELLOW;
            4 => Color::from_rgb(255, 255, 0),
            // CYAN;
            5 => Color::from_rgb(0, 255, 255),
            // MAGENTA;
            6 => Color::from_rgb(255, 0, 255),
            // ORANGE;
            7 => Color::from_rgb(255, 165, 0),
            // BLACK;
            8 => Color::from_rgb(0, 0, 0),
            _ => panic!("Unknown color in AnimStep {num}", num = self.1),
        }
    }
}

#[derive(Debug)]
pub enum TetrominoType {
    Square,
    LShape,
    LShapeReverse,
    IShape,
    SShape,
    SShapeReverse,
    HalfCross,
    CornerShape,
}

impl TetrominoType {
    fn from_block_num(num: u32) -> Self {
        use TetrominoType::*;
        match num {
            0 => Square,
            1 => LShape,
            2 => LShapeReverse,
            3 => IShape,
            4 => SShape,
            5 => SShapeReverse,
            6 => HalfCross,
            7 => CornerShape,
            _ => panic!("Unknown block number {num} for TetrominoType."),
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C, x: u32, y: u32, color: &Color, rotation: u8) {
        use TetrominoType::*;
        match self {
            Square => {
                canvas.set(x, y, color);
                canvas.set(x + 1, y, color);
                canvas.set(x, y - 1, color);
                canvas.set(x + 1, y - 1, color);
            }
            LShape => {
                if rotation == 0 {
                    canvas.set(x, y, color);
                    canvas.set(x + 1, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x, y - 2, color);
                }
                if rotation == 1 {
                    canvas.set(x, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x + 2, y - 1, color);
                }
                if rotation == 2 {
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x + 1, y - 2, color);
                    canvas.set(x, y - 2, color);
                }
                if rotation == 3 {
                    canvas.set(x, y, color);
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 2, y, color);
                    canvas.set(x + 2, y - 1, color);
                }
            }
            LShapeReverse => {
                if rotation == 0 {
                    canvas.set(x, y, color);
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x + 1, y - 2, color);
                }
                if rotation == 1 {
                    canvas.set(x, y, color);
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 2, y, color);
                    canvas.set(x, y - 1, color);
                }
                if rotation == 2 {
                    canvas.set(x, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x, y - 2, color);
                    canvas.set(x + 1, y - 2, color);
                }
                if rotation == 3 {
                    canvas.set(x, y - 1, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x + 2, y - 1, color);
                    canvas.set(x + 2, y, color);
                }
            }
            IShape => {
                if rotation == 0 || rotation == 2 {
                    // Horizontal
                    canvas.set(x, y, color);
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 2, y, color);
                    canvas.set(x + 3, y, color);
                }
                if rotation == 1 || rotation == 3 {
                    // Vertical
                    canvas.set(x, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x, y - 2, color);
                    canvas.set(x, y - 3, color);
                }
            }
            SShape => {
                if rotation == 0 || rotation == 2 {
                    canvas.set(x + 1, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x, y - 2, color);
                }
                if rotation == 1 || rotation == 3 {
                    canvas.set(x, y, color);
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x + 2, y - 1, color);
                }
            }
            SShapeReverse => {
                if rotation == 0 || rotation == 2 {
                    canvas.set(x, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x + 1, y - 2, color);
                }
                if rotation == 1 || rotation == 3 {
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 2, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x + 1, y - 1, color);
                }
            }
            HalfCross => {
                if rotation == 0 {
                    canvas.set(x, y, color);
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 2, y, color);
                    canvas.set(x + 1, y - 1, color);
                }
                if rotation == 1 {
                    canvas.set(x, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x, y - 2, color);
                    canvas.set(x + 1, y - 1, color);
                }
                if rotation == 2 {
                    canvas.set(x + 1, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x + 2, y - 1, color);
                }
                if rotation == 3 {
                    canvas.set(x + 1, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x + 1, y - 2, color);
                }
            }
            CornerShape => {
                if rotation == 0 {
                    canvas.set(x, y, color);
                    canvas.set(x + 1, y, color);
                    canvas.set(x, y - 1, color);
                }
                if rotation == 1 {
                    canvas.set(x, y, color);
                    canvas.set(x, y - 1, color);
                    canvas.set(x + 1, y - 1, color);
                }
                if rotation == 2 {
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 1, y - 1, color);
                    canvas.set(x, y - 1, color);
                }
                if rotation == 3 {
                    canvas.set(x, y, color);
                    canvas.set(x + 1, y, color);
                    canvas.set(x + 1, y - 1, color);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Tetromino {
    x: u32,
    y: u32,
    tetromino_type: TetrominoType,
    color: Color,
    rotation: u8,
    y_end: u32,
}

impl Tetromino {
    pub fn is_finished(&self) -> bool {
        self.y == self.y_end
    }

    pub fn move_down(&mut self) {
        if !self.is_finished() {
            self.y += 1;
        }
    }

    pub fn from_anim_step(step: &AnimStep, x: u32, y_offset: u32) -> Self {
        Self {
            tetromino_type: TetrominoType::from_block_num(step.0),
            x: x + step.2,
            y: y_offset,
            color: step.as_color(),
            rotation: step.4,
            y_end: y_offset + step.3,
        }
    }

    pub fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.tetromino_type
            .draw(canvas, self.x, self.y, &self.color, self.rotation);
    }
}
