use macroquad::{prelude::*};

const WIDTH:  i32 = 1920;
const HEIGHT: i32 = 1080;
const SCALE: f32 = 10.0;

const KE: f32 = 9_000_000_000.0;

struct Particle {
    pos: Vec2,

    charge: f32,
}

impl Particle {
    fn new(position: Vec2, charge: f32) -> Self {
        Self {
            pos: position,
            charge: charge,
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
                    field: Vec2::ZERO,
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
    field: Vec2,
}

#[macroquad::main(window_conf())]
async fn main() {
    let p1 = Particle::new(Vec2 { x: -10.0, y:  10.0 }, -1.0);
    let p2 = Particle::new(Vec2 { x:  10.0, y: -10.0 },  1.0);

    let mut particles = Vec::new();

    particles.push(p1);
    particles.push(p2);

    let mut grid = Grid::new(64);

    show_mouse(false);
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        // logic
        for cell in &mut grid.cells {
            cell.field = Vec2::ZERO;
        }

        for i in 0..grid.height {
            for j in 0..grid.length {
                for p in &particles {
                    let cell = grid.cells.get_mut((i * grid.length + j) as usize).unwrap();

                    let point = cell.pos;

                    cell.field += electric_field_at(p, point);
                }
            }
        }

        let p1 = particles.get_mut(0).unwrap();
        let (x, y) = mouse_position();
        let mouse_pos = reverse_projection(Vec2 { x, y }, SCALE);
        p1.pos = mouse_pos;

        // draw
        clear_background(BLACK);

        let max = 5000.0;
        let min = 0.0;
        for cell in &grid.cells {
            let pos = project_to_screen(cell.pos, SCALE);

            // tone it down a bit
            //let field = cell.field / (KE / 100.0);
            let field = cell.field;
            let module = field.length().sqrt();

            let t = (module - min) / (max - min);
            let color = Color::new(t, 0.0, 1.0-t, 1.0);

            // make sure the lines arent REALLY big
            let field = field.clamp_length(0.0, 2.0);

            let vec_end = project_to_screen(cell.pos + field, SCALE);

            draw_line(pos.x, pos.y, vec_end.x, vec_end.y, 1.0, color);
        }

        for p in &particles {
            let part_pos = project_to_screen(p.pos, SCALE);
            draw_circle(part_pos.x, part_pos.y, 5.0, BLUE);
        }

        next_frame().await
    }
}

fn electric_field_at(part: &Particle, point: Vec2) -> Vec2 {
    // E = K * Q / d^2
    let dist_sqrd = part.pos.distance_squared(point);

    let r = (point - part.pos).normalize();

    r * KE * part.charge / dist_sqrd
}

fn project_to_screen(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x:  p.x * scale + (WIDTH  as f32 / 2.0),
        y: -p.y * scale + (HEIGHT as f32 / 2.0)
    }
}

fn reverse_projection(p: Vec2, scale: f32) -> Vec2 {
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
