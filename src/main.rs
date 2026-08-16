use macroquad::{prelude::*};
use electric_field::{reverse_projection, project_to_screen, grid, particle, SCALE, WIDTH, HEIGHT};

#[macroquad::main(window_conf())]
async fn main() {
    let p1 = particle::Particle::new(Vec2 { x: -10.0, y:  10.0 }, -1.0);
    let p2 = particle::Particle::new(Vec2 { x:  10.0, y: -10.0 },  1.0);

    let mut particles = Vec::new();

    particles.push(p1);
    particles.push(p2);

    let mut grid = grid::Grid::new(64);

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

                    cell.field += p.electric_field_at(point);
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
