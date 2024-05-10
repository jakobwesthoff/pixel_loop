use std::collections::VecDeque;
use std::time::Duration;

use anyhow::{Context, Result};
use pixel_loop::{Canvas, Color};

//   int blocktype;  // Number of the block type
//   int color; // Color of the brick
//   int x_pos;      // x-position (starting from the left number staring point) where the brick should be placed
//   int y_stop;     // y-position (1-16, where 16 is the last line of the matrix) where the brick should stop falling
//   int num_rot;    // Number of 90-degree (clockwise) rotations a brick is turned from the standard position
#[derive(Clone)]
struct AnimStep(u32, u32, u32, u32, u8);

impl AnimStep {
    const ZERO: &'static [AnimStep] = &[
        AnimStep(2, 5, 4, 16, 0),
        AnimStep(4, 7, 2, 16, 1),
        AnimStep(3, 4, 0, 16, 1),
        AnimStep(6, 6, 1, 16, 1),
        AnimStep(5, 1, 4, 14, 0),
        AnimStep(6, 6, 0, 13, 3),
        AnimStep(5, 1, 4, 12, 0),
        AnimStep(5, 1, 0, 11, 0),
        AnimStep(6, 6, 4, 10, 1),
        AnimStep(6, 6, 0, 9, 1),
        AnimStep(5, 1, 1, 8, 1),
        AnimStep(2, 5, 3, 8, 3),
    ];

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
enum TetrominoType {
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
struct Tetromino {
    x: u32,
    y: u32,
    tetromino_type: TetrominoType,
    color: Color,
    rotation: u8,
    y_end: u32,
}

impl Tetromino {
    fn is_finished(&self) -> bool {
        self.y == self.y_end
    }

    fn from_anim_step(step: &AnimStep, x: u32, y_offset: u32) -> Self {
        Self {
            tetromino_type: TetrominoType::from_block_num(step.0),
            x: x + step.2,
            y: y_offset,
            color: step.as_color(),
            rotation: step.4,
            y_end: y_offset + step.3,
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.tetromino_type
            .draw(canvas, self.x, self.y, &self.color, self.rotation);
    }
}

#[derive(Default)]
struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,
    digits_anim: Vec<VecDeque<AnimStep>>,
    digits_active: Vec<Option<Tetromino>>,
    digits_fallen: Vec<Vec<Tetromino>>,
    updates_skipped: usize,
}

fn main() -> Result<()> {
    let width = 64;
    let height = 48;
    let scale = 10;

    let context = pixel_loop::init_tao_window("tetromino_time", width * scale, height * scale)
        .context("create tao window")?;
    let canvas =
        pixel_loop::init_pixels(&context, width, height).context("initialize pixel canvas")?;

    let mut state = State::default();
    state.digits_anim = vec![
        VecDeque::from(AnimStep::ZERO.to_vec()),
        VecDeque::from(AnimStep::ZERO.to_vec()),
        VecDeque::from(AnimStep::ZERO.to_vec()),
        VecDeque::from(AnimStep::ZERO.to_vec()),
        VecDeque::from(AnimStep::ZERO.to_vec()),
        VecDeque::from(AnimStep::ZERO.to_vec()),
    ];
    state.digits_active = vec![None, None, None, None, None, None];
    state.digits_fallen = vec![vec![], vec![], vec![], vec![], vec![], vec![]];

    pixel_loop::run_with_tao_and_pixels(
        state,
        context,
        canvas,
        |e, s, canvas| {
            s.updates_called += 1;
            let char_width = 9;
            // UPDATE BEGIN
            s.updates_skipped += 1;
            if s.updates_skipped <= 10 {
                s.updates_skipped += 1;
                return Ok(());
            } else {
                s.updates_skipped = 0;
            }

            for i in 0..s.digits_active.len() {
                match s.digits_active[i] {
                    Some(ref mut tetromino) => {
                        tetromino.y += 1;
                        if tetromino.is_finished() {
                            let tetromino = s.digits_active[i].take().unwrap();
                            s.digits_fallen[i].push(tetromino);
                        }
                    }
                    None => {
                        if let Some(next_step) = s.digits_anim[i].pop_front() {
                            s.digits_active[i] = Some(Tetromino::from_anim_step(
                                &next_step,
                                (i * char_width) as u32,
                                8,
                            ));
                        }
                    }
                }
            }
            // UPDATE END

            Ok(())
        },
        |e, s, canvas, dt| {
            let width = canvas.width();
            let height = canvas.height();

            // RENDER BEGIN
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));
            for digit in &s.digits_fallen {
                for tetromino in digit {
                    tetromino.draw(canvas);
                }
            }
            for candidate in &s.digits_active {
                if let Some(tetromino) = candidate {
                    tetromino.draw(canvas);
                }
            }
            // RENDER END

            s.renders_called += 1;
            s.time_passed += dt;
            if s.time_passed > Duration::from_secs(1) {
                println!("Update FPS: {:.2}", s.updates_called as f64 / 1f64);
                println!("Render FPS: {:.2}", s.renders_called as f64 / 1f64);
                s.updates_called = 0;
                s.renders_called = 0;
                s.time_passed = Duration::default();
            }

            canvas.blit()?;

            Ok(())
        },
        |e, s, canvas, _, event| Ok(()),
    );
}
