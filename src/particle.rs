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
    pub scale: f32,
    pub color: Color,
    pub active: bool,
}

impl Particle {
    pub fn new(init_pos: Vec2<f32>, init_dir: f32) -> tetra::Result<Particle> {
        let mut rng = rand::thread_rng();
        let speed = rng.gen_range(3.0, 5.0);
        let delta_dir = rng.gen_range(-PI / 6.0, PI / 6.0);

        Ok(Particle {
            position: init_pos,
            direction: PI + init_dir + delta_dir,
            theta: 0.0,
            speed: speed,
            scale: 3.0,
            color: Color::rgba(1.0, 0.0, 0.0, 1.0),
            active: true,
        })
    }
    
    pub fn update(&mut self, ctx: &mut Context) {
        self.position.x += self.speed * self.direction.cos();
        self.position.y += self.speed * self.direction.sin();
        let w = window::get_width(ctx) as f32;
        let h = window::get_height(ctx) as f32;

        self.color.a -= 0.1;
        if self.position.x < 0.0 || self.position.x > w || self.position.y < 0.0 ||self.position.y > h {
            self.active = false;
        }
    }
}
