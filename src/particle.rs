use macroquad::{prelude::*};
use crate::KE;
use crate::{project_to_screen, SCALE};

pub struct Particle {
    pub pos: Vec2,
    pub charge: f32,
}

impl Particle {
    pub fn new(position: Vec2, charge: f32) -> Self {
        Self {
            pos: position,
            charge: charge,
        }
    }

    pub fn electric_field_at(&self, point: Vec2) -> Vec2 {
        // E = K * Q / d^2
        let dist_sqrd = self.pos.distance_squared(point);

        let r = (point - self.pos).normalize();

        r * KE * self.charge / dist_sqrd
    }

    pub fn draw(&self) {
        let part_pos = project_to_screen(self.pos, SCALE);
        draw_circle(part_pos.x, part_pos.y, 1.0 * SCALE, BLUE);
    }
}

