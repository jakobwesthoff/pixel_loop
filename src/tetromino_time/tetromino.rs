use std::sync::OnceLock;

use pixel_loop::{Canvas, Color, InMemoryCanvas};

fn block_canvas() -> &'static InMemoryCanvas {
    static CANVAS: OnceLock<InMemoryCanvas> = OnceLock::new();
    CANVAS.get_or_init(|| {
        InMemoryCanvas::from_in_memory_image(include_bytes!("./assets/tetromino_block_16.png"))
            .expect("can't load base tetromino block asset")
    })
}

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
    x_pos: i64,
    y_stop: u32,
    rotation: u8,
}

impl AnimStep {
    // As we "stole" those animations from:
    // https://github.com/n00dles/esp_p10_tetris_clock/blob/master/src/numbers.h
    // we need some sort of conversion from his struct to ours.
    //
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
            x_pos: x_pos as i64,
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

    fn draw<TargetCanvas: Canvas, BlockCanvas: Canvas>(
        &self,
        canvas: &mut TargetCanvas,
        block: &BlockCanvas,
        x: i64,
        y: i64,
        color: &Color,
        rotation: u8,
    ) {
        use TetrominoType::*;
        match self {
            Square => {
                // canvas.set(x, y, color);
                // canvas.set(x + 1, y, color);
                // canvas.set(x, y - 1, color);
                // canvas.set(x + 1, y - 1, color);
                canvas.blit(block, x, y, Some(color));
                canvas.blit(block, x + block.width() as i64, y, Some(color));
                canvas.blit(block, x, y - block.width() as i64, Some(color));
                canvas.blit(
                    block,
                    x + block.width() as i64,
                    y - block.height() as i64,
                    Some(color),
                );
            }
            LShape => {
                if rotation == 0 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(block, x, y - block.height() as i64 * 2, Some(color));
                }
                if rotation == 1 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(
                        block,
                        x + block.width() as i64 * 2,
                        y - block.height() as i64,
                        Some(color),
                    );
                }
                if rotation == 2 {
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64 * 2,
                        Some(color),
                    );
                    canvas.blit(block, x, y - block.height() as i64 * 2, Some(color));
                }
                if rotation == 3 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x + block.width() as i64 * 2, y, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64 * 2,
                        y - block.height() as i64,
                        Some(color),
                    );
                }
            }
            LShapeReverse => {
                if rotation == 0 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64 * 2,
                        Some(color),
                    );
                }
                if rotation == 1 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x + block.width() as i64 * 2, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                }
                if rotation == 2 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(block, x, y - block.height() as i64 * 2, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64 * 2,
                        Some(color),
                    );
                }
                if rotation == 3 {
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(
                        block,
                        x + block.width() as i64 * 2,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(block, x + block.width() as i64 * 2, y, Some(color));
                }
            }
            IShape => {
                if rotation == 0 || rotation == 2 {
                    // Horizontal
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x + block.width() as i64 * 2, y, Some(color));
                    canvas.blit(block, x + block.width() as i64 * 3, y, Some(color));
                }
                if rotation == 1 || rotation == 3 {
                    // Vertical
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(block, x, y - block.height() as i64 * 2, Some(color));
                    canvas.blit(block, x, y - block.height() as i64 * 3, Some(color));
                }
            }
            SShape => {
                if rotation == 0 || rotation == 2 {
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(block, x, y - block.height() as i64 * 2, Some(color));
                }
                if rotation == 1 || rotation == 3 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(
                        block,
                        x + block.width() as i64 * 2,
                        y - block.height() as i64,
                        Some(color),
                    );
                }
            }
            SShapeReverse => {
                if rotation == 0 || rotation == 2 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64 * 2,
                        Some(color),
                    );
                }
                if rotation == 1 || rotation == 3 {
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x + block.width() as i64 * 2, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                }
            }
            HalfCross => {
                if rotation == 0 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x + block.width() as i64 * 2, y, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                }
                if rotation == 1 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(block, x, y - block.height() as i64 * 2, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                }
                if rotation == 2 {
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(
                        block,
                        x + block.width() as i64 * 2,
                        y - block.height() as i64,
                        Some(color),
                    );
                }
                if rotation == 3 {
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64 * 2,
                        Some(color),
                    );
                }
            }
            CornerShape => {
                if rotation == 0 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                }
                if rotation == 1 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                }
                if rotation == 2 {
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                    canvas.blit(block, x, y - block.height() as i64, Some(color));
                }
                if rotation == 3 {
                    canvas.blit(block, x, y, Some(color));
                    canvas.blit(block, x + block.width() as i64, y, Some(color));
                    canvas.blit(
                        block,
                        x + block.width() as i64,
                        y - block.height() as i64,
                        Some(color),
                    );
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Tetromino {
    x: i64,
    y: i64,
    tt: TetrominoType,
    tcolor: TetrominoColor,
    rotation: u8,
    y_stop: i64,
    speed: f64,
    acceleration: f64,
    max_speed: f64,
}

impl Tetromino {
    pub fn is_finished(&self) -> bool {
        self.y == self.y_stop
    }

    pub fn update<R: rand::Rng>(&mut self, rand: &mut R) {
        self.speed = self.speed + self.acceleration;
        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        }

        let mut movement = self.speed.floor() as i64;

        if rand.gen::<f64>() < self.speed - self.speed.floor() {
            movement += 1;
        }

        self.y += movement;

        if self.y > self.y_stop {
            self.y = self.y_stop;
            self.speed = 0.0;
        }
    }

    pub fn from_anim_step<R: rand::Rng>(
        step: AnimStep,
        rand: &mut R,
        x: u32,
        y_offset: i64,
    ) -> Self {
        // @TODO: Do not reference block canvas here directly!
        // @TODO: Extract the falling into some sort of base behaviour?
        Self {
            tt: step.tt,
            x: x as i64 + step.x_pos * block_canvas().width() as i64,
            y: y_offset,
            tcolor: step.tcolor,
            rotation: step.rotation,
            y_stop: y_offset + step.y_stop as i64 * block_canvas().height() as i64,
            speed: rand.gen::<f64>() * 0.2 + 0.1,
            acceleration: rand.gen::<f64>() * 0.1 + 0.1,
            max_speed: 7.0,
        }
    }

    pub fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.tt.draw(
            canvas,
            block_canvas(),
            self.x,
            self.y,
            self.tcolor.as_color(),
            self.rotation,
        );
    }
}
