use tetra::graphics::{self, BufferUsage, Mesh, Vertex, VertexBuffer, Color, DrawParams};
use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::math::Vec2;

mod spaceship;
use spaceship::*;

mod bullet;
use bullet::*;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;



struct GameState {
    spaceship: Spaceship,
    bullet_list: Vec<Bullet>,
    bullet_mesh: Mesh,
    spaceship_mesh: Mesh,
        
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        // Bullet Mesh
        let (pos_a, uv_a) = (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
        let (pos_b, uv_b) = (Vec2::new(2.5, 2.5), Vec2::new(0.5, 0.5));
        let (pos_c, uv_c) = (Vec2::new(0.0, 5.0), Vec2::new(0.0, 1.0));
        let bullet_vertices = &[
            Vertex::new(pos_a, uv_a, Color::WHITE),
            Vertex::new(pos_b, uv_b, Color::WHITE),
            Vertex::new(pos_c, uv_c, Color::WHITE),
        ];

        let bullet_mesh = VertexBuffer::with_usage(ctx, bullet_vertices, BufferUsage::Static)?.into_mesh();

        // Spaceship Mesh
        let (pos_e, uv_e) = (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
        let (pos_f, uv_f) = (Vec2::new(25.0, 10.0), Vec2::new(0.0, 1.0));
        let (pos_g, uv_g) = (Vec2::new(0.0, 20.0), Vec2::new(1.0, 1.0));
        let (pos_h, uv_h) = (Vec2::new(5.0, 10.0), Vec2::new(1.0, 0.0));

        let spaceship_vertices = &[
            //triangle 1
            Vertex::new(pos_e, uv_e, Color::WHITE),
            Vertex::new(pos_f, uv_f, Color::WHITE),
            Vertex::new(pos_h, uv_h, Color::WHITE),
            //trinagle 2
            Vertex::new(pos_f, uv_f, Color::WHITE),
            Vertex::new(pos_g, uv_g, Color::WHITE),
            Vertex::new(pos_h, uv_h, Color::WHITE),
        ];
        let spaceship_mesh = VertexBuffer::with_usage(ctx, spaceship_vertices, BufferUsage::Static)?.into_mesh();


        Ok(GameState { 
            bullet_list: Vec::new(),
            bullet_mesh: bullet_mesh,
            spaceship: Spaceship::new(WIDTH * 0.5, HEIGHT * 0.5)?,
            spaceship_mesh: spaceship_mesh
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {

        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));

        for bullet in self.bullet_list.iter() {
            graphics::draw(
                ctx,
                &self.bullet_mesh,
                DrawParams::new()
                    .position(bullet.position)
                    .origin(Vec2::new(0.0, 2.5))
                    .scale(Vec2::new(bullet.scale, bullet.scale))
                    .color(bullet.color)
                    .rotation(bullet.theta),
            );
            
        }

        graphics::draw(
            ctx,
            &self.spaceship_mesh,
            DrawParams::new()
                .position(self.spaceship.position)
                .origin(Vec2::new(12.5, 10.0))
                .scale(Vec2::new(self.spaceship.scale, self.spaceship.scale))
                .rotation(self.spaceship.theta)
                .color(self.spaceship.color),
        );
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        self.spaceship.update();
        for b in self.bullet_list.iter_mut() {
            b.update();
        }

        if input::is_key_down(ctx, Key::Left) {
            self.spaceship.rotation_left();
        }
        else if input::is_key_down(ctx, Key::Right) {
            self.spaceship.rotation_right();
        }

        if input::is_key_pressed(ctx, Key::Space) {
            self.bullet_list.push(Bullet::new(self.spaceship.position, self.spaceship.theta)?);
            
        }

        if input::is_key_down(ctx, Key::Up) {
            self.spaceship.engine_on();
        } else {
            self.spaceship.engine_off();
        }

        // Debug 
        if input::is_key_pressed(ctx, Key::W) {
            self.spaceship.position = Vec2::new(WIDTH / 2.0, HEIGHT / 2.0);
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

