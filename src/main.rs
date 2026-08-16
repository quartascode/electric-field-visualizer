use std::usize;

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

struct Grid {
    length: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(grid_length: u32) -> Self {
        let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
        let grid_height = (grid_length as f32 / aspect_ratio).round() as u32;

        let size = WIDTH as f32 / (SCALE * grid_length as f32);
        let mut cells = Vec::new();

        let top_left = reverse_projection(Vec2::ZERO, SCALE);

        for i in 0..grid_height {
            for j in 0..grid_length {
                let x = (top_left.x + size * 1.0 * j as f32) + size * 0.5;
                let y = (top_left.y - size * 1.0 * i as f32) - size * 0.5;

                let cell = Cell {
                    pos: Vec2 { x, y },
                    tesla: 0.0,
                };

                cells.push(cell);
            }
        }

        Grid {
            length: grid_length,
            height: grid_height,
            cells: cells,
        }
    }
}

struct Cell {
    pos: Vec2,
    tesla: f32,
}

#[macroquad::main(window_conf())]
async fn main() {
    let p1 = Particle::new(Vec2 { x: -10.0, y:  10.0 }, Vec2 { x: 0.0, y:  0.1 }, 1.0,  10.0, BLUE);

    let mut particles = Vec::new();

    particles.push(p1);

    let mut grid = Grid::new(64);

    let vel = 10.0;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        // logic
        let particle = particles.get_mut(0).unwrap();
        particle.pos.y -= vel * get_frame_time();

        for i in 0..grid.height {
            for j in 0..grid.length {
                let cell = grid.cells.get_mut((i * grid.length + j) as usize).unwrap();

                let point = cell.pos;

                cell.tesla = electric_field_module(particle, point);
            }
        }

        // draw
        clear_background(BLACK);

        let max = 50.0;
        let min = 0.0;
        for cell in &grid.cells {
            let pos = project_to_screen(cell.pos, SCALE);

            let t = (cell.tesla - min) / (max - min);
            let color = Color::new(t, 0.0, 1.0 - t, 1.0);
            draw_circle(pos.x, pos.y, 15.0, color);
        }

        next_frame().await
    }
}

fn electric_field_module(part: &Particle, point: Vec2) -> f32 {
    // E = K * Q / d^2
    let dist_sqrd = part.pos.distance_squared(point);

    KE * part.charge / dist_sqrd
}

fn project_to_screen(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x:  p.x * scale + (WIDTH  as f32 / 2.0),
        y: -p.y * scale + (HEIGHT as f32 / 2.0)
    }
}

fn reverse_projection(p: Vec2, scale: f32) -> Vec2 {
    //25.6 - 43.2
    Vec2 {
        x: ( p.x - (WIDTH  as f32 / 2.0)) / scale,
        y: (-p.y + (HEIGHT as f32 / 2.0)) / scale
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
