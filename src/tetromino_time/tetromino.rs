use pixel_loop::{Canvas, Color};

#[derive(Clone, Debug)]
enum TetrominoColor {
    Red,
    Green,
    Blue,
    White,
    Yellow,
    Cyan,
    Magenta,
    Orange,
}

impl TetrominoColor {
    const RED: Color = Color::from_rgb(255, 0, 0);
    const GREEN: Color = Color::from_rgb(0, 255, 0);
    const BLUE: Color = Color::from_rgb(0, 0, 255);
    const WHITE: Color = Color::from_rgb(255, 255, 255);
    const YELLOW: Color = Color::from_rgb(255, 255, 0);
    const CYAN: Color = Color::from_rgb(0, 255, 255);
    const MAGENTA: Color = Color::from_rgb(255, 0, 255);
    const ORANGE: Color = Color::from_rgb(255, 165, 0);

    pub const fn from_num_color(num_color: u8) -> Self {
        use TetrominoColor::*;
        match num_color {
            0 => Red,
            1 => Green,
            2 => Blue,
            3 => White,
            4 => Yellow,
            5 => Cyan,
            6 => Magenta,
            7 => Orange,
            // As it is a const fn we can not panic here.
            _ => White,
        }
    }

    pub fn as_color(&self) -> &Color {
        use TetrominoColor::*;
        match self {
            Red => &Self::RED,
            Green => &Self::GREEN,
            Blue => &Self::BLUE,
            White => &Self::WHITE,
            Yellow => &Self::YELLOW,
            Cyan => &Self::CYAN,
            Magenta => &Self::MAGENTA,
            Orange => &Self::ORANGE,
        }
    }
}

#[derive(Clone)]
pub struct AnimStep {
    tt: TetrominoType,
    tcolor: TetrominoColor,
    x_pos: u32,
    y_stop: u32,
    rotation: u8,
}

impl AnimStep {
    // int blocktype;  // Number of the block type
    // int color; // Color of the brick
    // int x_pos;      // x-position (starting from the left number staring point) where the brick should be placed
    // int y_stop;     // y-position (1-16, where 16 is the last line of the matrix) where the brick should stop falling
    // int num_rot;
    pub const fn from_numeric(
        num_type: u32,
        num_color: u8,
        x_pos: u32,
        y_stop: u32,
        num_rot: u8,
    ) -> Self {
        Self {
            tt: TetrominoType::from_num_type(num_type),
            tcolor: TetrominoColor::from_num_color(num_color),
            x_pos,
            y_stop,
            rotation: num_rot,
        }
    }
}

#[derive(Debug, Clone)]
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
    const fn from_num_type(num: u32) -> Self {
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
            // We can not panic here as this is const fn
            _ => Square,
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
    tt: TetrominoType,
    tcolor: TetrominoColor,
    rotation: u8,
    y_stop: u32,
}

impl Tetromino {
    pub fn is_finished(&self) -> bool {
        self.y == self.y_stop
    }

    pub fn move_down(&mut self) {
        if !self.is_finished() {
            self.y += 1;
        }
    }

    pub fn from_anim_step(step: AnimStep, x: u32, y_offset: u32) -> Self {
        Self {
            tt: step.tt,
            x: x + step.x_pos,
            y: y_offset,
            tcolor: step.tcolor,
            rotation: step.rotation,
            y_stop: y_offset + step.y_stop,
        }
    }

    pub fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.tt.draw(
            canvas,
            self.x,
            self.y,
            self.tcolor.as_color(),
            self.rotation,
        );
    }
}
