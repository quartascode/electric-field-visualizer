use macroquad::{prelude::*};

const WIDTH:  i32 = 1280;
const HEIGHT: i32 = 720;
const SCALE: f32 = 25.0;

const KE: f32 = 1_000.0;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    accel: Vec2,

    mass: f32,
    charge: f32,
    radius: f32,
    color: Color,
}

impl Particle {
    fn new(position: Vec2, initial_vel: Vec2, mass: f32, charge: f32, color: Color) -> Self {
        if mass < 0.0 {
            panic!("Negative mass isn't allowed");
        }

        Self {
            pos: position,
            vel: initial_vel,
            accel: Vec2 { x: 0.0, y: 0.0 },
            mass: mass,
            charge: charge,
            radius: 0.5,
            color: color,
        }
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let p1 = Particle::new(Vec2 { x: -10.0, y:  10.0 }, Vec2 { x: 0.0, y:  0.1 }, 1.0,  1.0, BLUE);
    let p2 = Particle::new(Vec2 { x:  10.0, y: -10.0 }, Vec2 { x: 0.0, y: -0.1 }, 1.0,  1.0, RED);

    let mut particles = Vec::new();

    particles.push(p1);
    particles.push(p2);

    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        clear_background(BLACK);

        next_frame().await
    }
}

fn project_to_screen(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x:  p.x * scale + (WIDTH  as f32 / 2.0),
        y: -p.y * scale + (HEIGHT as f32 / 2.0)
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "electric field".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}
