use macroquad::{prelude::*};

const WIDTH:  i32 = 1280;
const HEIGHT: i32 = 720;
const SCALE: f32 = 100.0;

const KE: f32 = 9_000_000_000.0;

struct Particle {
    pos: Vec2,
    prev_pos: Vec2,
    force: Vec2,

    mass: f32,
    inv_mass: f32,
    charge: f32,
}

impl Particle {
    fn new(position: Vec2, mass: f32, charge: f32) -> Self {
        if mass < 0.0 {
            panic!("Negative mass isn't allowed");
        }

        Self {
            pos: position,
            prev_pos: Vec2 { x: 0.0, y: 0.0 },
            force: Vec2 { x: 0.0, y: 0.0 },
            mass: mass,
            inv_mass: 1.0 / mass,
            charge: charge,
        }
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let p1 = Particle {
        pos: Vec2 { x: -10.0, y: 10.0 },
        force: Vec2 { x: 0.0, y: 0.0 },
        mass: 1.0,
        charge: 0.000
    };

    let p2 = Particle {
        pos: Vec2 { x: 10.0, y: -10.0 },
        force: Vec2 { x: 0.0, y: 0.0 },
        mass: 1.0,
        charge: 0.000
    };

    let mut particles = Vec::new();

    particles.push(p1);
    particles.push(p2);

    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        update(&mut particles, get_frame_time());

        draw(&particles);

        next_frame().await
    }
}

fn update(particles: &mut [Particle], dt: f32) {
    for i in 0..particles.len() {
        for j in i+1..particles.len() {
            let (first, last) = particles.split_at_mut(j);

            let p1 = &mut first[i];
            let p2 = &mut last[0];

            let force = eletric_force(p1, p2);

            let dir = p2.pos - p1.pos;
            let force = force * dir;

            p1.force =  force;
            p2.force = -force;
        }
    }

    for p in particles {
        dbg!(p.pos);
        integrate(p, dt);

    }
}


fn integrate(p: &mut Particle, dt: f32) {
    let accel = p.force / p.mass;
    let vel = accel * dt;

    p.pos += vel * dt;
}

fn eletric_force(p1: &Particle, p2: &Particle) -> f32 {
    // Fel = K*|Q|*|q| / d^2
    let a = KE * p1.charge * p2.charge;
    let d = p1.pos.distance(p2.pos);

    match d {
        0.0 => 0.0,
        _   => a / (d * d)
    }
}

fn draw(parts: &[Particle]) {
    for p in parts {
        let radius = 0.1;
        let screen_pos = project_to_screen(p.pos, SCALE);

        draw_circle(screen_pos.x, screen_pos.y, radius * SCALE, BLUE);
    }
}

fn project_to_screen(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x:  p.x * scale + (WIDTH  as f32 / 2.0),
        y: -p.y * scale + (HEIGHT as f32 / 2.0)
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "electric field".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}
