use macroquad::{prelude::*};
use crate::{project_to_screen, reverse_projection};

use crate::WIDTH;
use crate::HEIGHT;
use crate::SCALE;

pub struct Grid {
    pub length: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(grid_length: u32) -> Self {
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

    pub fn draw(&self) {
        let max = 5000.0;
        let min = 0.0;
        for cell in &self.cells {
            let screen_pos = project_to_screen(cell.pos, SCALE);

            // tone it down a bit
            //let field = cell.field / (KE / 100.0);
            let field = cell.field;
            let module = field.length().sqrt();
            if module < 1.0 {
                continue;
            }

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
    }
}

pub struct Cell {
    pub pos: Vec2,
    pub field: Vec2,
}
