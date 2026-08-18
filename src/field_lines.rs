use crate::particle::Particle;
use std::f32::consts::PI;
use macroquad::{prelude::*};
use crate::{blue_to_red_color, project_to_screen, SCALE};

const AMOUNT: i32 = 36;
const MAX_DEPTH: i32 = 500;
const STEP_SIZE: f32 = 1.00;

pub fn draw(particles: &Vec<Particle>) {
    for p in particles {
        if p.charge <= 0.0 {
            continue;
        }
        let pos = p.pos;
        for i in 0..AMOUNT {
            let angle = i as f32 * 2.0*PI/AMOUNT as f32;
            let x = pos.x + angle.cos();
            let y = pos.y + angle.sin();

            let mut start = Vec2 { x, y };

            for _ in 0..MAX_DEPTH {
                let dst_sqrd = p.pos.distance_squared(start);
                if dst_sqrd < 0.9 {
                    break;
                }

                let mut field = Vec2::ZERO;
                for p in particles {
                    field += p.electric_field_at(start);
                }

                let module = field.length().sqrt();
                let color = blue_to_red_color(1.2, 450.0, module);

                field = field.normalize() * STEP_SIZE;

                let dir = start + field;

                let screen_end = project_to_screen(dir, SCALE);
                let screen_start = project_to_screen(start, SCALE);

                draw_line(screen_start.x, screen_start.y, screen_end.x, screen_end.y, 2.0, color);

                start = dir;
            }
        }
    }
}
