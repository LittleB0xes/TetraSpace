use tetra::graphics::{self, BufferUsage, Mesh, Vertex, VertexBuffer, Color, Texture, Rectangle, DrawParams};
use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::window;
use tetra::math::Vec2;


const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

struct Spaceship {
    position: Vec2<f32>,
    speed: Vec2<f32>,
    theta: f32,
    scale: f32,
    ship: Mesh,
    engine: bool,
}

impl Spaceship {
    fn new(ctx: &mut Context) -> tetra::Result<Spaceship> {
        let (pos_e, uv_e) = (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
        let (pos_f, uv_f) = (Vec2::new(25.0, 10.0), Vec2::new(0.0, 1.0));
        let (pos_g, uv_g) = (Vec2::new(0.0, 20.0), Vec2::new(1.0, 1.0));
        let (pos_h, uv_h) = (Vec2::new(5.0, 10.0), Vec2::new(1.0, 0.0));

        let spaceship_vertices = &[
            //triangle 1
            Vertex::new(pos_e, uv_e, Color::RED),
            Vertex::new(pos_f, uv_f, Color::RED),
            Vertex::new(pos_h, uv_h, Color::RED),
            //trinagle 2
            Vertex::new(pos_f, uv_f, Color::BLUE),
            Vertex::new(pos_g, uv_g, Color::BLUE),
            Vertex::new(pos_h, uv_h, Color::BLUE),
        ];
        let mut spaceship_mesh = VertexBuffer::with_usage(ctx, spaceship_vertices, BufferUsage::Static)?.into_mesh();

        Ok(Spaceship {
            position: Vec2::new(WIDTH / 2.0, HEIGHT / 2.0),
            speed: Vec2::new(0.0,0.0),
            theta: 0.0,
            scale: 2.0,
            engine: false,
            ship: spaceship_mesh,
        })
    }

    fn rotation_left(&mut self) {
        self.theta -= 0.05;
    }

    fn rotation_right(&mut self) {
        self.theta += 0.05;
    }

    fn update(&mut self) {
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
    fn engine_on(&mut self) {
        self.engine = true;
    }
    fn engine_off(&mut self) {
        self.engine = false;
    }
}

struct GameState {
    timer: f32,
    spaceship: Spaceship,
        
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {


        //mesh.set_texture(Texture::new(ctx, "./resources/block.png")?);

        Ok(GameState { 
            timer: 0.0,
            spaceship: Spaceship::new(ctx)?,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let curve = self.timer.sin() + 2.0;

        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));

        graphics::draw(
            ctx,
            &self.spaceship.ship,
            DrawParams::new()
                .position(self.spaceship.position)
                .origin(Vec2::new(12.5, 10.0))
                .scale(Vec2::new(self.spaceship.scale, self.spaceship.scale))
                .rotation(self.spaceship.theta),
        );
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.timer += 0.01;

        self.spaceship.update();

        if input::is_key_down(ctx, Key::Left) {
            self.spaceship.rotation_left();
        }
        else if input::is_key_down(ctx, Key::Right) {
            self.spaceship.rotation_right();
        }

        if input::is_key_down(ctx, Key::Up) {
            self.spaceship.engine_on();
        } else {
            self.spaceship.engine_off();
        }
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("My Awesome Tetra Rust Game", WIDTH as i32, HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

