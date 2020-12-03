use tetra::Context;
use tetra::math::Vec2;
use rand::Rng;
use tetra::graphics::Color;
use tetra::window;
use std::f32::consts::PI;

pub struct Particle {
    pub position: Vec2<f32>,
        speed: f32,
        direction: f32,
    pub theta: f32,
        rotation_speed: f32,
    pub scale: f32,
    pub color: Color,
    pub active: bool,
        deviation: f32,
}

impl Particle {
    pub fn new(init_pos: Vec2<f32>, init_dir: f32) -> tetra::Result<Particle> {
        let mut rng = rand::thread_rng();
        let speed = rng.gen_range(3.0, 5.0);
        let delta_dir = rng.gen_range(-PI / 6.0, PI / 6.0);
        let rotation_speed = rng.gen_range(0.03, 0.06);
        let mut sign = -1.0;
        if rng.gen_ratio(1, 2) {
            sign = 1.0;
        }

        let r = rng.gen_range(0.8, 1.0);
        let g = rng.gen_range(0.4, 0.6);
        let b = rng.gen_range(0.0, 0.02);

        Ok(Particle {
            position: init_pos,
            direction: PI + init_dir + delta_dir,
            theta: 0.0,
            speed: speed,
            scale: 3.0,
            color: Color::rgba(r, g, b, 1.0),
            active: true,
            rotation_speed: sign * rotation_speed,
            deviation: 0.0,
        })
    }
    
    pub fn update(&mut self, ctx: &mut Context) {
    self.deviation += 0.08 * self.rotation_speed;
        self.direction += self.deviation; 
        self.speed *= 0.98;
        self.position.x += self.speed * self.direction.cos();
        self.position.y += self.speed * self.direction.sin();
        let w = window::get_width(ctx) as f32;
        let h = window::get_height(ctx) as f32;

        self.color.a -= 0.02;
        self.scale *= 0.98;
        self.theta += self.rotation_speed;

        if self.position.x < 0.0 || self.position.x > w || self.position.y < 0.0 ||self.position.y > h || self.color.a <= 0.0 {
            self.active = false;
        }
    }
}
