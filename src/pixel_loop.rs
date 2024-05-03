use std::time::{Duration, Instant};

use pixels::{Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::Event;
use tao::event_loop::EventLoop;
use tao::window::WindowBuilder;

// Inpsired by: https://gafferongames.com/post/fix_your_timestep/
pub fn pixel_loop<S>(
    mut state: S,
    update_fps: usize,
    update: fn(&mut S),
    render: fn(&mut S, Duration),
) {
    if update_fps == 0 {
        panic!("Designated FPS for updates needs to be > 0");
    }

    let mut accum: Duration = Duration::new(0, 0);
    let mut current_time = Instant::now();
    let mut last_time;

    let update_dt = Duration::from_nanos((1_000_000_000f64 / update_fps as f64).round() as u64);

    loop {
        last_time = current_time;
        current_time = Instant::now();
        let mut dt = current_time - last_time;

        // Escape hatch if update calls take to long in order to not spiral into
        // death
        if dt > Duration::from_millis(100) {
            dt = Duration::from_millis(100);
        }

        while accum > update_dt {
            update(&mut state);
            accum -= update_dt;
        }

        render(&mut state, dt);

        accum += dt;
    }
}

pub fn pixel_loop_tao<S: 'static>(
    mut state: S,
    (width, height): (u32, u32),
    update_fps: usize,
    update: fn(&mut S, u32, u32),
    render: fn(&mut S, Duration, u32, u32, &mut Pixels),
) {
    if update_fps == 0 {
        panic!("Designated FPS for updates needs to be > 0");
    }

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(width, height);
        WindowBuilder::new()
            .with_title("Hello Pixels/Tao")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };

    let mut accum: Duration = Duration::new(0, 0);
    let mut current_time = Instant::now();
    let mut last_time = Instant::now();

    let update_dt = Duration::from_nanos((1_000_000_000f64 / update_fps as f64).round() as u64);

    event_loop.run(move |event, _, control_flow| {
        match event {
            // Update internal state and request a redraw
            Event::MainEventsCleared => {
                last_time = current_time;
                current_time = Instant::now();
                let mut dt = current_time - last_time;

                // Escape hatch if update calls take to long in order to not spiral into
                // death
                if dt > Duration::from_millis(100) {
                    dt = Duration::from_millis(100);
                }

                while accum > update_dt {
                    update(&mut state, width, height);
                    accum -= update_dt;
                }
                render(&mut state, dt, width, height, &mut pixels);
                accum += dt;

                if let Err(err) = pixels.render() {
                    panic!("Pixels render error");
                }
            }

            _ => {}
        }
    });
}
