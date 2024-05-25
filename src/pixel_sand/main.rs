use anyhow::{Context, Result};
use pixel_loop::{Canvas, Color, HslColor, RenderableCanvas};
use rand::Rng;
use std::time::Duration;
use winit::event::{WindowEvent, ElementState, VirtualKeyCode, MouseButton, KeyboardInput, Event};

#[derive(Clone, PartialEq)]
struct Sand {
    color: Color,
    acceleration: f32,
    speed: f32,
    max_speed: f32,
}

impl Sand {
    fn new<R: rand::Rng + ?Sized>(rand: &mut R, base_color: &Color) -> Self {
        let color = Self::sand_color_variation(rand, base_color);
        // @TODO: Fix that something not moving is assumed to be not updated any
        // more. And then change this one to * 0.3 + 0.3
        let acceleration = rand.gen::<f32>() * 0.3 + 1.0;
        let speed = rand.gen::<f32>() * 1.3 + 0.2;
        let max_speed = rand.gen::<f32>() * 2.0 + 2.0;
        Self {
            color,
            acceleration,
            speed,
            max_speed,
        }
    }

    fn update_state(&mut self) {
        self.speed += self.acceleration;
        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        }
    }

    fn get_steps<R: rand::Rng + ?Sized>(&self, rand: &mut R) -> usize {
        let whole_part = self.speed.floor();
        let fractional = self.speed - whole_part;

        // Use the fractional part as the probability to execute the movement
        // step.
        // No idea if this is good idea, but seems to work ;)
        if rand.gen::<f32>() < fractional {
            (whole_part + 1.0) as usize
        } else {
            whole_part as usize
        }
    }

    fn sand_color_variation<R: rand::Rng + ?Sized>(rand: &mut R, color: &Color) -> Color {
        let hsl = color.as_hsl();

        let range = 10f64;

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

    fn get_steps<R: rand::Rng + ?Sized>(&self, rand: &mut R) -> usize {
        match self {
            Particle::Empty => 0,
            Particle::Sand(ref sand) => sand.get_steps(rand),
        }
    }
}

struct ParticleGrid {
    particles: Vec<Particle>,
    particles_to_update: Vec<usize>,
    width: u32,
    height: u32,
}

impl ParticleGrid {
    fn new(width: u32, height: u32) -> Self {
        Self {
            particles: vec![Particle::Empty; (width * height) as usize],
            particles_to_update: vec![],
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
        let index = (y * self.width + x) as usize;

        self.particles[index] = particle;
        self.particles_to_update.push(index);
    }

    fn add_sand_particles<R: rand::Rng + ?Sized>(
        &mut self,
        rand: &mut R,
        cx: u32,
        cy: u32,
        r: u32,
        base_color: &Color,
        probability: f64,
    ) {
        let r = r as i64;

        for dy in -r..=r {
            for dx in -r..=r {
                let x = cx as i64 + dx;
                let y = cy as i64 + dy;

                if x < 0 || y < 0 {
                    continue;
                }

                if dx * dx + dy * dy <= r * r {
                    if rand.gen::<f64>() < probability {
                        let particle = Particle::Sand(Sand::new(rand, base_color));
                        self.set_particle(x as u32, y as u32, particle);
                    }
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

    fn update_particles<R: rand::Rng + ?Sized>(&mut self, rand: &mut R) {
        let mut particles_to_update = std::mem::replace(&mut self.particles_to_update, Vec::new());
        particles_to_update.sort_unstable();
        for i in particles_to_update.iter().rev().cloned() {
            self.particles[i].update_state();
            let steps = self.particles[i].get_steps(rand);

            let mut working_index = i;
            let mut needs_further_update = false;
            for _ in 0..steps {
                let new_working_index = self.execute_step(working_index);
                if new_working_index == working_index {
                    break;
                } else {
                    working_index = new_working_index;
                    needs_further_update = true;
                }
            }

            if needs_further_update {
                self.particles_to_update.push(working_index);
            }
        }
    }
}

#[derive(Debug)]
enum AutoMode {
    Disabled,
    Waterfall,
    Fountain,
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
    w_is_pressed: bool,
    f_is_pressed: bool,
    auto_mode: AutoMode,
    emitter_collection: EmitterCollection,
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
            w_is_pressed: false,
            f_is_pressed: false,
            auto_mode: AutoMode::Disabled,
            emitter_collection: EmitterCollection::None,
            grid: ParticleGrid::new(width, height),
        }
    }
}

#[derive(Debug, Clone)]
struct Emitter {
    x: u32,
    y: u32,
    color: Color,
    probability: f64,
    radius: u32,
}

impl Emitter {
    fn emit<R: rand::Rng + ?Sized>(&self, rand: &mut R, grid: &mut ParticleGrid) {
        if self.radius == 1 {
            if rand.gen::<f64>() < self.probability {
                let particle = Particle::Sand(Sand::new(rand, &self.color));
                grid.set_particle(self.x, self.y, particle);
            }
        } else {
            grid.add_sand_particles(
                rand,
                self.x,
                self.y,
                self.radius,
                &self.color,
                self.probability,
            )
        }
    }
}

trait Emitting {
    fn emit<R: rand::Rng>(&self, rand: &mut R, grid: &mut ParticleGrid);
}

enum EmitterCollection {
    None,
    Waterfall(EmitterWatterfall),
    Fountains(EmitterFountains),
}

impl Emitting for EmitterCollection {
    fn emit<R: rand::Rng>(&self, rand: &mut R, grid: &mut ParticleGrid) {
        match self {
            EmitterCollection::None => {}
            EmitterCollection::Waterfall(ref ec) => ec.emit(rand, grid),
            EmitterCollection::Fountains(ref ec) => ec.emit(rand, grid),
        }
    }
}

struct EmitterFountains {
    emitters: Vec<Emitter>,
    height: u32,
    width: u32,
    fountains: u32,
    radius: u32,
    probability: f64,
}

impl EmitterFountains {
    fn new<R: rand::Rng>(
        rand: &mut R,
        width: u32,
        height: u32,
        fountains: u32,
        radius: u32,
        probability: f64,
    ) -> Self {
        let mut emitters = vec![];
        let color = Color::from_rgb(rand.gen::<u8>(), rand.gen::<u8>(), rand.gen::<u8>());
        let spacing = width / fountains;
        for (x_start, x_end) in (0..width)
            .step_by(spacing as usize)
            .zip((spacing as u32..width).step_by(spacing as usize))
        {
            let x = rand.gen_range(x_start..x_end);
            let y = rand.gen_range(0..height);

            emitters.push(Emitter {
                x,
                y,
                probability: probability * rand.gen::<f64>(),
                color: color.clone(),
                radius,
            });
        }

        Self {
            width,
            height,
            radius,
            fountains,
            emitters,
            probability,
        }
    }
}
impl Emitting for EmitterFountains {
    fn emit<R: rand::Rng>(&self, rand: &mut R, grid: &mut ParticleGrid) {
        for emitter in self.emitters.iter() {
            emitter.emit(rand, grid);
        }
    }
}

#[derive(Default, Debug)]
struct EmitterWatterfall {
    emitters: Vec<Emitter>,
    height: u32,
    width: u32,
}

impl EmitterWatterfall {
    fn new<R: rand::Rng>(rand: &mut R, width: u32, height: u32) -> Self {
        let mut emitters = vec![];

        let mut color = Color::from_rgb(rand.gen::<u8>(), rand.gen::<u8>(), rand.gen::<u8>());
        for x in 0..width {
            if rand.gen::<f64>() < 0.01 {
                color = Color::from_rgb(rand.gen::<u8>(), rand.gen::<u8>(), rand.gen::<u8>());
            }
            for y in 0..height {
                if rand.gen::<f64>() < 0.05 {
                    emitters.push(Emitter {
                        x,
                        y,
                        probability: 0.7 * rand.gen::<f64>(),
                        color: color.clone(),
                        radius: 1,
                    });
                }
            }
        }

        Self {
            width,
            height,
            emitters,
        }
    }
}
impl Emitting for EmitterWatterfall {
    fn emit<R: rand::Rng>(&self, rand: &mut R, grid: &mut ParticleGrid) {
        for emitter in self.emitters.iter() {
            emitter.emit(rand, grid);
        }
    }
}

fn main() -> Result<()> {
    let width = 640;
    let height = 480;

    let context = pixel_loop::winit::init_window("pixel loop", width, height, false)
        .context("create tao window")?;
    let canvas =
        pixel_loop::winit::init_pixels(&context, width, height).context("initialize pixel canvas")?;

    let state = State::new(width, height);

    pixel_loop::winit::run(
        state,
        context,
        canvas,
        |e, s, canvas| {
            s.updates_called += 1;
            let sand_color = Color::from_rgb(226, 202, 118);
            // UPDATE BEGIN
            if s.w_is_pressed {
                match s.auto_mode {
                    AutoMode::Disabled => {
                        s.auto_mode = AutoMode::Waterfall;
                    }
                    AutoMode::Waterfall => {
                        s.auto_mode = AutoMode::Disabled;
                    }
                    AutoMode::Fountain => {
                        s.auto_mode = AutoMode::Waterfall;
                    }
                }
                if let AutoMode::Waterfall = s.auto_mode {
                    // Create new emitterbar on every activation on auto mode
                    s.emitter_collection = EmitterCollection::Waterfall(EmitterWatterfall::new(
                        &mut e.rand,
                        s.grid.width,
                        8,
                    ));
                }
                println!("Auto Mode: {auto_mode:?}", auto_mode = s.auto_mode);
            }

            if s.f_is_pressed {
                match s.auto_mode {
                    AutoMode::Disabled => {
                        s.auto_mode = AutoMode::Fountain;
                    }
                    AutoMode::Waterfall => {
                        s.auto_mode = AutoMode::Fountain;
                    }
                    AutoMode::Fountain => {
                        s.auto_mode = AutoMode::Disabled;
                    }
                }
                if let AutoMode::Fountain = s.auto_mode {
                    // Create new fountains on every activation on auto mode
                    s.emitter_collection = EmitterCollection::Fountains(EmitterFountains::new(
                        &mut e.rand,
                        s.grid.width,
                        50,
                        6,
                        15,
                        0.05,
                    ));
                }
                println!("Auto Mode: {auto_mode:?}", auto_mode = s.auto_mode);
            }

            match s.auto_mode {
                AutoMode::Disabled => {}
                AutoMode::Waterfall => {
                    if e.rand.gen::<f64>() < 0.01 {
                        s.emitter_collection = EmitterCollection::Waterfall(
                            EmitterWatterfall::new(&mut e.rand, s.grid.width, 8),
                        );
                    }
                    s.emitter_collection.emit(&mut e.rand, &mut s.grid);
                }
                AutoMode::Fountain => {
                    if e.rand.gen::<f64>() < 0.005 {
                        s.emitter_collection = EmitterCollection::Fountains(EmitterFountains::new(
                            &mut e.rand,
                            s.grid.width,
                            50,
                            6,
                            15,
                            0.05,
                        ));
                    }
                    s.emitter_collection.emit(&mut e.rand, &mut s.grid);
                }
            }

            if s.button_pressed {
                s.grid.add_sand_particles(
                    &mut e.rand,
                    s.cursor_position.0,
                    s.cursor_position.1,
                    10,
                    &sand_color,
                    0.3,
                );
            }

            s.grid.update_particles(&mut e.rand);
            // UPDATE END

            s.w_is_pressed = false;
            s.f_is_pressed = false;
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

            canvas.render()?;

            Ok(())
        },
        |e, s, canvas, _, input, event| {
            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Released,
                                virtual_keycode: Some(VirtualKeyCode::W),
                            ..
                        } => {
                            s.w_is_pressed = true;
                        }
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(VirtualKeyCode::F),
                            ..
                        } => {
                            s.f_is_pressed = true;
                        }
                        _ => {}
                    },
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
