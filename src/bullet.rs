use tetra::Context;
use tetra::math::Vec2;


pub struct Bullet {
    pub position: Vec2<f32>,
        speed: f32,
    pub theta: f32,
    pub scale: f32,
}

impl Bullet {
    pub fn new(ctx: &mut Context, init_pos: Vec2<f32>, init_dir: f32) -> tetra::Result<Bullet> {
        Ok(Bullet {
            position: init_pos,
            speed: 5.0,
            theta: init_dir,
            scale: 2.0,

        })
    }

    pub fn update(&mut self) {
        self.position.x += self.speed * self.theta.cos();
        self.position.y += self.speed * self.theta.sin();
    }
}
