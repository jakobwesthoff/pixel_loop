use anyhow::Result;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, terminal};
use pixel_loop::canvas::CrosstermCanvas;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
use pixel_loop::{Canvas, Color, HslColor, RenderableCanvas};

struct Particle {
    position: (f64, f64),
    speed: (f64, f64),
    acceleration: (f64, f64),
    fading: f64,
    lifetime: f64,
    color: Color,
    dimensions: (u32, u32),
}

impl Particle {
    pub fn new(x: i64, y: i64, width: u32, height: u32, color: Color) -> Self {
        Self {
            position: (x as f64, y as f64),
            speed: (0.0, 0.0),
            acceleration: (0.0, 0.0),
            fading: 0.0,
            lifetime: 1.0,
            color,
            dimensions: (width, height),
        }
    }

    pub fn with_speed(self, x: f64, y: f64) -> Self {
        Self {
            speed: (x, y),
            ..self
        }
    }

    pub fn with_acceleration(self, x: f64, y: f64) -> Self {
        Self {
            acceleration: (x, y),
            ..self
        }
    }

    pub fn with_fading(self, fading: f64) -> Self {
        Self { fading, ..self }
    }

    pub fn update(&mut self) {
        if self.lifetime <= 0.0 {
            return;
        }

        self.speed.0 += self.acceleration.0;
        self.speed.1 += self.acceleration.1;

        self.position.0 += self.speed.0;
        self.position.1 += self.speed.1;

        self.lifetime -= self.fading;
    }

    pub fn render<C: Canvas>(&self, canvas: &mut C) -> Result<()> {
        if self.lifetime <= 0.0 {
            return Ok(());
        }

        // @HACK: PixelLoop with CrosstermCanvas does not support proper alpha
        // blending at the moment. Therefore we calculate the coler against a
        // given base (black) and the lifetime as opacity and apply it.
        let render_color = Color::from_rgb(
            (self.color.r as f64 * self.lifetime) as u8,
            (self.color.g as f64 * self.lifetime) as u8,
            (self.color.b as f64 * self.lifetime) as u8,
        );

        canvas.filled_rect(
            self.position.0.round() as i64,
            self.position.1.round() as i64,
            self.dimensions.0,
            self.dimensions.1,
            &render_color,
        );
        Ok(())
    }

    pub fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }
}

struct Firework {
    rocket: Option<Particle>,
    effect: Vec<Particle>,
    effect_base_color: HslColor,
}

impl Firework {
    pub fn new(x: i64, y: i64, effect_base_color: Color) -> Self {
        let rocket = Some(
            Particle::new(x, y, 1, 3, Color::from_rgb(255, 255, 255))
                // Rocket flies upwards with gravity pulling it down.
                // Initial speed slightly randomized.
                .with_speed(0.0, -2.0 - rand::random::<f64>() * -1.0)
                .with_acceleration(0.0, 0.02),
        );

        Self {
            rocket,
            effect: vec![],
            effect_base_color: effect_base_color.as_hsl(),
        }
    }

    pub fn update(&mut self) {
        if let Some(ref mut rocket) = self.rocket {
            rocket.update();

            if rocket.speed.1 >= -0.2 {
                // Rocket has reached its peak and is now exploding.
                // Create a bunch of particles to simulate the explosion.
                for _ in 0..25 {
                    let x = rocket.position.0 as i64;
                    let y = rocket.position.1 as i64;
                    let width = 1;
                    let height = 1;
                    // Randomize color based on the base color of the rocket. using the hsl form
                    // of the color.
                    let color = HslColor::new(
                        self.effect_base_color.h,
                        self.effect_base_color.s + (rand::random::<f64>() - 0.5) * 20.0,
                        self.effect_base_color.l + (rand::random::<f64>() - 0.5) * 40.0,
                    );

                    let particle = Particle::new(x, y, width, height, color.into())
                        .with_speed(
                            (rand::random::<f64>() - 0.5) * 1.0,
                            (rand::random::<f64>() - 0.9) * 1.0,
                        )
                        .with_acceleration(0.0, 0.02)
                        .with_fading(0.01);
                    self.effect.push(particle);
                }
                self.rocket = None;
            }
        }

        for particle in &mut self.effect {
            particle.update();
        }
    }

    pub fn render<C: Canvas>(&self, canvas: &mut C) -> Result<()> {
        if let Some(ref rocket) = self.rocket {
            rocket.render(canvas)?;
        }

        for particle in &self.effect {
            particle.render(canvas)?;
        }

        Ok(())
    }

    pub fn is_dead(&self) -> bool {
        self.rocket.is_none() && self.effect.iter().all(|p| p.is_dead())
    }
}

struct State {
    fireworks: Vec<Firework>,
}

impl State {
    fn new(width: u32, height: u32) -> Self {
        Self { fireworks: vec![] }
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

    pixel_loop::run(
        60,
        state,
        input,
        canvas,
        |e, s, input, canvas| {
            let width = canvas.width();
            let height = canvas.height();

            if input.is_key_pressed(KeyboardKey::Q) {
                // @HACK until we refactored PixelLoop to allow for a clean
                // exit.
                let mut stdout = std::io::stdout();
                execute!(
                    stdout,
                    Clear(ClearType::All), // Clear all on screen
                    cursor::MoveTo(0, 0),  // Reset cursor position
                    Print("\x1b[!p"),      // Soft terminal reset (DECSTR)
                    Print("\x1bc"),        // Full terminal reset (RIS)
                )?;
                crossterm::terminal::disable_raw_mode()?;
                std::process::exit(0);
            }

            // eprintln!("Active fireworks: {}", s.fireworks.len());

            // Remove dead fireworks.
            s.fireworks.retain(|f| !f.is_dead());

            // Add a new rocket with with 5% chance.
            if rand::random::<f64>() < 0.05 {
                let x = (rand::random::<u32>() % width) as i64;
                let y = height as i64;
                let effect_base_color = Color::from_rgb(
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                );

                s.fireworks.push(Firework::new(x, y, effect_base_color));
            }

            for firework in &mut s.fireworks {
                firework.update();
            }

            Ok(())
        },
        |e, s, i, canvas, dt| {
            // RENDER BEGIN
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));

            for firework in &s.fireworks {
                firework.render(canvas)?;
            }

            // RENDER END

            canvas.render()?;

            Ok(())
        },
    )?;
    Ok(())
}
