use macroquad::{prelude::*};

pub const WIDTH:  i32 = 1920;
pub const HEIGHT: i32 = 1080;
pub const SCALE: f32 = 10.0;

pub const KE: f32 = 9_000_000_000.0;

pub mod grid;
pub mod particle;
pub mod field_lines;

pub fn project_to_screen(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x:  p.x * scale + (WIDTH  as f32 / 2.0),
        y: -p.y * scale + (HEIGHT as f32 / 2.0)
    }
}

pub fn reverse_projection(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x: ( p.x - (WIDTH  as f32 / 2.0)) / scale,
        y: (-p.y + (HEIGHT as f32 / 2.0)) / scale
    }
}

pub fn blue_to_red_color(min: f32, max: f32, value: f32) -> Color {
    let t = (value - min) / (max - min);
    Color::new(t, 0.0, 1.0-t, 1.0)
}
