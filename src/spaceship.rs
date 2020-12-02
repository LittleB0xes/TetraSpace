use tetra::math::Vec2;
use tetra::graphics::Color;


pub struct Spaceship {
    pub position: Vec2<f32>,
        speed: Vec2<f32>,
    pub theta: f32,
    pub scale: f32,
        engine: bool,
    pub color: Color,
}

impl Spaceship {
    pub fn new(x: f32, y: f32) -> tetra::Result<Spaceship> {

        Ok(Spaceship {
            position: Vec2::new(x, y),
            speed: Vec2::new(0.0,0.0),
            theta: 0.0,
            scale: 1.0,
            engine: false,
            color: Color::RED,
        })
    }

    pub fn rotation_left(&mut self) {
        self.theta -= 0.05;
    }

    pub fn rotation_right(&mut self) {
        self.theta += 0.05;
    }

    pub fn update(&mut self) {
        if self.engine == true && self.speed.magnitude_squared() < 100.0 {
            self.speed.x += 0.5 * self.theta.cos();
            self.speed.y += 0.5 * self.theta.sin();
        } else if !self.engine {
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
