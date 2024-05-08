use anyhow::{Context, Result};
use pixel_loop::{Canvas, Color, HslColor};
use std::time::Duration;
use tao::event::{ElementState, Event, MouseButton, WindowEvent};

#[derive(Clone, PartialEq)]
struct Sand {
    color: Color,
    velocity: f32,
    speed: f32,
    max_speed: f32,
}

impl Sand {
    fn new<R: rand::Rng + ?Sized>(rand: &mut R, base_color: &Color) -> Self {
        let color = Self::sand_color_variation(rand, base_color);
        Self {
            color,
            velocity: 0.1,
            speed: 1.0,
            max_speed: 5.0,
        }
    }

    fn update_state(&mut self) {
        self.speed += self.velocity;
        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        }
    }

    fn get_steps(&self) -> usize {
        self.speed.round() as usize
    }

    fn sand_color_variation<R: rand::Rng + ?Sized>(rand: &mut R, color: &Color) -> Color {
        let hsl = color.as_hsl();

        let range = 0.1;

        let ds = range * rand.gen::<f64>() - range / 2.0;
        let dl = range * rand.gen::<f64>() - range / 2.0;

        HslColor::new(hsl.h, hsl.s + ds, hsl.l + dl).into()
    }
}

#[derive(Clone, PartialEq)]
enum Particle {
    Empty,
    Sand(Sand),
}

impl Particle {
    fn update_state(&mut self) {
        match self {
            Particle::Empty => {}
            Particle::Sand(ref mut sand) => sand.update_state(),
        }
    }

    fn get_steps(&self) -> usize {
        match self {
            Particle::Empty => 0,
            Particle::Sand(ref sand) => sand.get_steps(),
        }
    }
}

struct ParticleGrid {
    particles: Vec<Particle>,
    width: u32,
    height: u32,
}

impl ParticleGrid {
    fn new(width: u32, height: u32) -> Self {
        Self {
            particles: vec![Particle::Empty; (width * height) as usize],
            width,
            height,
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        let empty_color = Color::from_rgb(0, 0, 0);

        for (i, p) in self.particles.iter().enumerate() {
            match p {
                Particle::Empty => canvas.set_range(i..i + 1, std::slice::from_ref(&empty_color)),
                Particle::Sand(ref sand) => {
                    canvas.set_range(i..i + 1, std::slice::from_ref(&sand.color))
                }
            }
        }
    }

    fn set_particle(&mut self, x: u32, y: u32, particle: Particle) {
        if x >= self.width || y >= self.height {
            eprintln!(
                "WARNING: tried to set particle outside of bounds {x}x{y} ({width}x{height})",
                width = self.width,
                height = self.height
            );
            return;
        }

        self.particles[(y * self.width + x) as usize] = particle;
    }

    fn set_circle(&mut self, cx: u32, cy: u32, r: u32, particle: Particle) {
        let r = r as i64;

        for dy in -r..=r {
            for dx in -r..=r {
                let x = cx as i64 + dx;
                let y = cy as i64 + dy;

                if x < 0 || y < 0 {
                    continue;
                }

                if dx * dx + dy * dy <= r * r {
                    self.set_particle(x as u32, y as u32, particle.clone());
                }
            }
        }
    }

    fn execute_step(&mut self, i: usize) -> usize {
        match self.particles[i] {
            Particle::Empty => i,
            Particle::Sand(ref sand) => {
                let below = i + self.width as usize;
                let below_left = i + self.width as usize - 1;
                let below_right = i + self.width as usize + 1;
                if below < self.particles.len() && Particle::Empty == self.particles[below] {
                    self.particles.swap(i, below);
                    below
                } else if below_left < self.particles.len()
                    && Particle::Empty == self.particles[below_left]
                {
                    self.particles.swap(i, below_left);
                    below_left
                } else if below_right < self.particles.len()
                    && Particle::Empty == self.particles[below_right]
                {
                    self.particles.swap(i, below_right);
                    below_right
                } else {
                    i
                }
            }
        }
    }

    fn update_particles(&mut self) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].update_state();
            let steps = self.particles[i].get_steps();

            let mut working_index = i;
            for _ in 0..steps {
                let new_working_index = self.execute_step(working_index);
                if new_working_index == working_index {
                    break;
                } else {
                    working_index = new_working_index;
                }
            }
        }
    }
}

struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,
    box_position: (isize, isize),
    box_direction: (isize, isize),
    box_size: (usize, usize),
    button_pressed: bool,
    cursor_position: (u32, u32),
    grid: ParticleGrid,
}

impl State {
    fn new(width: u32, height: u32) -> Self {
        Self {
            updates_called: Default::default(),
            renders_called: Default::default(),
            time_passed: Default::default(),
            box_position: Default::default(),
            box_direction: (2, 2),
            box_size: (50, 50),
            button_pressed: false,
            cursor_position: (0, 0),
            grid: ParticleGrid::new(width, height),
        }
    }
}

fn main() -> Result<()> {
    let width = 640;
    let height = 480;

    let context =
        pixel_loop::init_tao_window("pixel loop", width, height).context("create tao window")?;
    let canvas =
        pixel_loop::init_pixels(&context, width, height).context("initialize pixel canvas")?;

    let state = State::new(width, height);

    pixel_loop::run_with_tao_and_pixels(
        state,
        context,
        canvas,
        |e, s, canvas| {
            s.updates_called += 1;
            let sand_color = Color::from_rgb(226, 202, 118);
            // UPDATE BEGIN
            if s.button_pressed {
                s.grid.set_circle(
                    s.cursor_position.0,
                    s.cursor_position.1,
                    10,
                    Particle::Sand(Sand::new(&mut e.rand, &sand_color)),
                );
            }

            s.grid.update_particles();
            // UPDATE END
            Ok(())
        },
        |e, s, canvas, dt| {
            let width = canvas.width();
            let height = canvas.height();

            // RENDER BEGIN
            s.grid.draw(canvas);
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
        |e, s, canvas, _, event| {
            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state,
                        ..
                    } => {
                        if state == &ElementState::Pressed {
                            s.button_pressed = true;
                        } else {
                            s.button_pressed = false;
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let pixel_position = canvas
                            .physical_pos_to_canvas_pos(position.x, position.y)
                            .unwrap_or((0, 0));
                        s.cursor_position = pixel_position;
                    }
                    _ => {}
                },
                _ => {}
            }
            Ok(())
        },
    );
}
