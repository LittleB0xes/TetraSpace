use tetra::math::Vec2;
use tetra::graphics::Color;


pub struct Bullet {
    pub position: Vec2<f32>,
        speed: f32,
    pub theta: f32,
    pub scale: f32,
    pub color: Color,
}

impl Bullet {
    pub fn new(init_pos: Vec2<f32>, init_dir: f32) -> tetra::Result<Bullet> {
        Ok(Bullet {
            position: init_pos,
            speed: 10.0,
            theta: init_dir,
            scale: 1.0,
            color: Color::RED,

        })
    }

    pub fn update(&mut self) {
        self.position.x += self.speed * self.theta.cos();
        self.position.y += self.speed * self.theta.sin();
    }
}
