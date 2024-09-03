use anyhow::Result;
use crossterm::terminal;
use pixel_loop::canvas::CrosstermCanvas;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
use pixel_loop::{Canvas, Color, RenderableCanvas};

const PLAYFIELD_WIDTH: usize = 100;
const PLAYFIELD_HEIGHT: usize = 100;

struct Paddle {
    position: (usize, usize),
    dimensions: (usize, usize),
    color: Color,
}

impl Paddle {
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        Self {
            position: (PLAYFIELD_WIDTH / 2 - width / 2, PLAYFIELD_HEIGHT - height),
            dimensions: (width, height),
            color,
        }
    }

    pub fn render<C: Canvas>(&self, canvas: &mut C) -> Result<()> {
        canvas.filled_rect(
            self.position.0 as i64,
            self.position.1 as i64,
            self.dimensions.0 as u32,
            self.dimensions.1 as u32,
            &self.color,
        );
        Ok(())
    }

    pub fn move_left(&mut self) {
        if self.position.0 > 0 {
            self.position.0 -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.position.0 < PLAYFIELD_WIDTH - self.dimensions.0 {
            self.position.0 += 1;
        }
    }
}

struct Ball {
    position: (usize, usize),
    dimensions: (usize, usize),
    speed: (isize, isize),
    color: Color,
    attached: bool,
}

impl Ball {
    pub fn new(paddle: &Paddle, width: usize, height: usize, color: Color) -> Self {
        let position = (
            paddle.position.0 + paddle.dimensions.0 / 2 - width / 2,
            paddle.position.1 - height,
        );

        Self {
            position,
            dimensions: (width, height),
            speed: (0, 0),
            color,
            attached: true,
        }
    }

    pub fn render<C: Canvas>(&self, canvas: &mut C) -> Result<()> {
        canvas.filled_rect(
            self.position.0 as i64,
            self.position.1 as i64,
            self.dimensions.0 as u32,
            self.dimensions.1 as u32,
            &self.color,
        );
        Ok(())
    }

    pub fn update(&mut self, paddle: &Paddle) {
        if self.attached {
            let position = (
                paddle.position.0 + paddle.dimensions.0 / 2 - self.dimensions.0 / 2,
                paddle.position.1 - self.dimensions.1,
            );

            self.position = position;

            return;
        }

        let new_position = (
            self.position.0 as isize + self.speed.0,
            self.position.1 as isize + self.speed.1,
        );

        // LEFT + RIGHT
        if new_position.0 < 0
            || new_position.0 > PLAYFIELD_WIDTH as isize - self.dimensions.0 as isize
        {
            self.speed.0 = self.speed.0 * -1;
        }

        // TOP
        if new_position.1 < 0 {
            self.speed.1 = self.speed.1 * -1;
        }

        // BOTTOM
        if new_position.1 > (PLAYFIELD_WIDTH - self.dimensions.1) as isize {
            self.attached = true;
            self.speed = (0, 0)
        }

        // Paddle colision
        if new_position.0 >= paddle.position.0 as isize
            && new_position.0 <= (paddle.position.0 + paddle.dimensions.0) as isize
            && new_position.1 >= paddle.position.1 as isize
        {
            let distance = new_position.0 - paddle.position.0 as isize;
            let scaled_distance = (distance as f64 / paddle.dimensions.0 as f64) * 2.0 - 1.0;
            let x_speed = (scaled_distance * 3.0).round() as isize;
            self.speed = (x_speed, -1);
        }

        // assgin and fix overflow positions
        self.position.0 = new_position.0.clamp(0, PLAYFIELD_WIDTH as isize) as usize;
        self.position.1 = new_position.1.clamp(0, PLAYFIELD_HEIGHT as isize) as usize;
    }
}

struct Brick {
    position: (usize, usize),
    dimensions: (usize, usize),
    color: Color,
    destroyed: bool,
}

impl Brick {
    pub fn new(x: usize, y: usize, width: usize, height: usize, color: Color) -> Self {
        Self {
            position: (x, y),
            dimensions: (width, height),
            color,
            destroyed: false,
        }
    }

    pub fn render<C: Canvas>(&self, canvas: &mut C) -> Result<()> {
        if self.destroyed {
            return Ok(());
        }

        canvas.filled_rect(
            self.position.0 as i64,
            self.position.1 as i64,
            self.dimensions.0 as u32,
            self.dimensions.1 as u32,
            &self.color,
        );
        Ok(())
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
    }

    pub fn colides_with_ball(&self, ball: &Ball) -> bool {
        !self.destroyed
            && ball.position.0 >= self.position.0
            && ball.position.0 <= self.position.0 + self.dimensions.0
            && ball.position.1 >= self.position.1
            && ball.position.1 <= self.position.1 + self.dimensions.1
    }
}

struct State {
    paddle: Paddle,
    ball: Ball,
    bricks: Vec<Brick>,
}

impl State {
    pub fn new() -> Self {
        let paddle = Paddle::new(17, 2, Color::from_rgb(255, 255, 255));
        Self {
            ball: Ball::new(&paddle, 1, 1, Color::from_rgb(255, 128, 0)),
            paddle,
            bricks: vec![],
        }
    }
}

fn main() -> Result<()> {
    let (terminal_width, terminal_height) = terminal::size()?;
    let width = terminal_width;
    let height = terminal_height * 2;

    let mut canvas = CrosstermCanvas::new(width, height);
    canvas.set_refresh_limit(120);

    let mut state = State::new();

    for y in 0..10 {
        for x in 0..10 {
            state.bricks.push(Brick::new(
                5 + x * (7 + 2),
                5 + y * (2 + 2),
                7,
                2,
                Color::from_rgb(255, 48, 128),
            ));
        }
    }

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

            if input.is_key_down(KeyboardKey::Left) {
                s.paddle.move_left();
            }

            if input.is_key_down(KeyboardKey::Right) {
                s.paddle.move_right();
            }

            if input.is_key_pressed(KeyboardKey::Space) {
                s.ball.attached = false;
                // straight up
                s.ball.speed = (0, -1)
            }

            for brick in s.bricks.iter_mut() {
                if brick.colides_with_ball(&s.ball) {
                    brick.destroy();
                    s.ball.speed.1 = s.ball.speed.1 * -1;
                }
            }

            s.ball.update(&s.paddle);

            Ok(())
        },
        |e, s, i, canvas, dt| {
            // RENDER BEGIN
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));

            canvas.filled_rect(
                0,
                0,
                PLAYFIELD_WIDTH as u32,
                PLAYFIELD_HEIGHT as u32,
                &Color::from_rgb(40, 40, 40),
            );

            for brick in &s.bricks {
                brick.render(canvas)?;
            }
            s.paddle.render(canvas)?;
            s.ball.render(canvas)?;

            // RENDER END

            canvas.render()?;

            Ok(())
        },
    )?;
    Ok(())
}
