use crate::particle::Particle;
use std::f32::consts::PI;
use macroquad::{prelude::*};
use crate::{blue_to_red_color, project_to_screen, SCALE};

pub fn draw(particles: &Vec<Particle>) {
    for p in particles {
        if p.charge <= 0.0 {
            continue;
        }
        let amount = 24;
        let max_depth = 2400;
        let step_size = 1.00;
        let pos = p.pos;
        for i in 0..amount {
            let angle = i as f32 * 2.0*PI/amount as f32;
            let x = pos.x + angle.cos();
            let y = pos.y + angle.sin();

            let mut start = Vec2 { x, y };
            'line: for _ in 0..max_depth {
                let dst_sqrd = p.pos.distance_squared(start);
                if dst_sqrd < 0.9 {
                    break 'line;
                }

                let mut field = Vec2::ZERO;
                for p in particles {
                    field += p.electric_field_at(start);
                }

                let module = field.length().sqrt();
                let color = blue_to_red_color(0.0, 5000.0, module);

                field = field.normalize() * step_size;

                let dir = start + field;

                let screen_end = project_to_screen(dir, SCALE);
                let screen_start = project_to_screen(start, SCALE);

                draw_line(screen_start.x, screen_start.y, screen_end.x, screen_end.y, 2.0, color);

                start = dir;
            }
        }
    }
}
