use macroquad::{prelude::*};
use electric_field::{reverse_projection, project_to_screen, grid, particle, SCALE, WIDTH, HEIGHT};

const GRID_AMOUNT_HORIZONTAL: u32 = 48;

#[macroquad::main(window_conf())]
async fn main() {
    let p1 = particle::Particle::new(Vec2 { x: -10.0, y:  10.0 }, -0.5);
    let p2 = particle::Particle::new(Vec2 { x:  10.0, y: -10.0 },  0.5);

    let mut particles = Vec::new();

    particles.push(p1);
    particles.push(p2);

    let mut grid = grid::Grid::new(GRID_AMOUNT_HORIZONTAL);

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
            let screen_pos = project_to_screen(cell.pos, SCALE);

            // tone it down a bit
            //let field = cell.field / (KE / 100.0);
            let field = cell.field;
            let module = field.length().sqrt();

            let t = (module - min) / (max - min);
            let color = Color::new(t, 0.0, 1.0-t, 1.0);

            // make sure the lines arent REALLY big
            let field = field.clamp_length(0.0, 2.0);

            let vec_end = cell.pos + field;
            let screen_vec_end = project_to_screen(vec_end, SCALE);

            let dir = (vec_end - cell.pos).normalize();
            let perp = Vec2 { x: -dir.y, y: dir.x };
            let l = 0.75;
            let c = vec_end - dir * l;
            let arrow1 = c + perp * l * 0.5;
            let arrow2 = c - perp * l * 0.5;
            let arrow1 = project_to_screen(arrow1, SCALE);
            let arrow2 = project_to_screen(arrow2, SCALE);

            let girth = 1.5;
            draw_line(screen_vec_end.x, screen_vec_end.y, arrow1.x, arrow1.y, girth, color);
            draw_line(screen_vec_end.x, screen_vec_end.y, arrow2.x, arrow2.y, girth, color);

            draw_line(screen_pos.x, screen_pos.y, screen_vec_end.x, screen_vec_end.y, girth, color);
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
