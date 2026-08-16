use macroquad::{prelude::*};
use crate::KE;

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
}

