use macroquad::{prelude::*};
use electric_field::{HEIGHT, SCALE, WIDTH, field_lines, grid, particle, reverse_projection};

const GRID_AMOUNT_HORIZONTAL: u32 = 64;

#[macroquad::main(window_conf())]
async fn main() {
    // render line fields?
    let mut render_mode: bool = false;

    let p1 = particle::Particle::new(Vec2 { x: -10.0, y:  10.0 },  0.005);
    let p2 = particle::Particle::new(Vec2 { x: -50.0, y: -25.0 }, -0.005);
    let p3 = particle::Particle::new(Vec2 { x:   0.0, y:  25.0 },  0.005);

    let mut particles = Vec::new();

    particles.push(p1);
    particles.push(p2);
    particles.push(p3);

    let mut grid = grid::Grid::new(GRID_AMOUNT_HORIZONTAL);

    loop {
        //input
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            render_mode = !render_mode;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let mouse_pos = reverse_projection(Vec2 { x, y }, SCALE);

            for p in &mut particles {
                let dst = p.pos.distance(mouse_pos);
                if dst <= 1.0 {
                    p.moving = !p.moving;
                }
            }
        }

        //logic
        for p in &mut particles {
            if p.moving {
                let (x, y) = mouse_position();
                let mouse_pos = reverse_projection(Vec2 { x, y }, SCALE);

                p.pos = mouse_pos;
            }
        }

        // draw
        clear_background(BLACK);

        if render_mode {
            field_lines::draw(&particles);
        } else {
            grid.calculate_cell_field(&particles);

            for cell in &grid.cells {
                cell.draw();
            }
        }

        for p in &particles {
            p.draw();
        }

        draw_fps();

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
