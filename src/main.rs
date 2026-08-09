use macroquad::{prelude::*};

const WIDTH:  i32 = 1280;
const HEIGHT: i32 = 720;
const SCALE: f32 = 20.0;

const KE: f32 = 9_000_000_000.0;

struct Particle {
    pos: Vec2,
    prev_pos: Vec2,
    accel: Vec2,

    mass: f32,
    inv_mass: f32,
    charge: f32,
    radius: f32,
}

impl Particle {
    fn new(position: Vec2, initial_vel: Vec2, mass: f32, charge: f32) -> Self {
        if mass < 0.0 {
            panic!("Negative mass isn't allowed");
        }

        Self {
            pos: position,
            prev_pos: position - initial_vel,
            accel: Vec2 { x: 0.0, y: 0.0 },
            mass: mass,
            inv_mass: 1.0 / mass,
            charge: charge,
            radius: 0.5,
        }
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let p1 = Particle::new(Vec2 { x: -10.0, y:  10.0 }, Vec2 { x: 0.0, y: 0.0 }, 1.0, 0.001);
    let p2 = Particle::new(Vec2 { x:  10.0, y: -10.0 }, Vec2 { x: 0.0, y: 0.0 }, 1.0, 0.001);

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

            let force = electric_force(p1, p2);

            let dir = p2.pos - p1.pos;
            let force = force * dir;

            let is_neg1 = p1.charge.is_sign_negative();
            let is_neg2 = p2.charge.is_sign_negative();

            if is_neg1 || !is_neg2 {
                p1.accel += force * p1.inv_mass;
                p2.accel -= force * p2.inv_mass;
            }
        }
    }

    for p in particles.iter_mut() {
        integrate(p, dt);
    }

    for i in 0..particles.len() {
        for j in i+1..particles.len() {
            let (first, last) = particles.split_at_mut(j);

            let p1 = &mut first[i];
            let p2 = &mut last[0];

            particle_collision(p1, p2);
        }
    }
}


fn integrate(p: &mut Particle, dt: f32) {
    p.pos = 2.0*p.pos - p.prev_pos + p.accel*dt*dt;
    p.prev_pos = p.pos;
    //p.accel = Vec2 { x: 0.0, y: 0.0 };
}

fn electric_force(p1: &Particle, p2: &Particle) -> f32 {
    // Fel = K*|Q|*|q| / d^2
    let a = KE * p1.charge * p2.charge;
    let d = p1.pos.distance(p2.pos);
    dbg!(d);

    if d < 20.1 {
        0.0
    } else {
        a / (d * d)
    }
}

fn particle_collision(p1: &mut Particle, p2: &mut Particle) {
    //let rest = (p1.rest * p2.rest).sqrt();

    let dst = p1.pos.distance(p2.pos);

    let overlap = p1.radius + p2.radius - dst;

    if overlap > 0.0 && dst != 0.0 {
        let n = (p2.pos - p1.pos) / dst;

        p1.pos += overlap * 0.5 * n;
        p2.pos -= overlap * 0.5 * n;
    }
}

fn draw(parts: &[Particle]) {
    for p in parts {
        let screen_pos = project_to_screen(p.pos, SCALE);

        draw_circle(screen_pos.x, screen_pos.y, p.radius * SCALE, BLUE);
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
