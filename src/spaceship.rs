use tetra::math::Vec2;
use tetra::Context;


pub struct Spaceship {
    pub position: Vec2<f32>,
        speed: Vec2<f32>,
    pub theta: f32,
    pub scale: f32,
        engine: bool,
}

impl Spaceship {
    pub fn new(ctx: &mut Context, view_width: f32, view_height: f32) -> tetra::Result<Spaceship> {

        Ok(Spaceship {
            position: Vec2::new(view_width / 2.0, view_height / 2.0),
            speed: Vec2::new(0.0,0.0),
            theta: 0.0,
            scale: 2.0,
            engine: false,
        })
    }

    pub fn rotation_left(&mut self) {
        self.theta -= 0.05;
    }

    pub fn rotation_right(&mut self) {
        self.theta += 0.05;
    }

    pub fn update(&mut self) {
        if self.engine == true {
            let acc = 0.5;
            self.speed.x += 0.5 * self.theta.cos();
            self.speed.y += 0.5 * self.theta.sin();
        } else {
            self.speed.x *= 0.98;
            self.speed.y *= 0.98;
        }
        self.position.x += self.speed.x;
        self.position.y += self.speed.y;
    }
    pub fn engine_on(&mut self) {
        self.engine = true;
    }
    pub fn engine_off(&mut self) {
        self.engine = false;
    }
}
