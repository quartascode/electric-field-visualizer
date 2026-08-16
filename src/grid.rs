use macroquad::{prelude::*};

use crate::WIDTH;
use crate::HEIGHT;
use crate::SCALE;

use crate::reverse_projection;

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
}

pub struct Cell {
    pub pos: Vec2,
    pub field: Vec2,
}
