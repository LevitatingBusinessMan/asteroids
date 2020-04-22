use ggez::{graphics, Context};

use std::vec::Vec;

mod movement;
use movement::Movement;

mod player;
use player::Player;

pub struct Laser {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    speed: f32,
    width: f32
}

impl Laser {
    pub fn new(x: f32, y: f32, rotation: f32) -> Laser {
        Laser {
            x,
            y,
            rotation,
            speed: 17.0,
            width: 15.0
        }
    }
    
    pub fn update(&mut self) {
        self.x += self.rotation.cos() * self.speed;
        self.y += self.rotation.sin() * self.speed;
    }

}

impl Draw for Laser {

    fn mesh(&self, ctx: &mut Context) -> ggez::GameResult<graphics::Mesh> {
        graphics::MeshBuilder::new()
        .line(
            &[
                [0.0,0.0],
                [15.0,0.0]
            ],
            2.0,
            graphics::WHITE
        )?
        .build(ctx)
    }

    fn draw_param(&self) -> graphics::DrawParam {
        graphics::DrawParam::new()
            .dest([self.x, self.y])
            .offset([0.5 * self.width, 0.0])
            .rotation(self.rotation)
    }
    
    fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        let mesh = self.mesh(ctx)?;
        let param = self.draw_param();
        graphics::draw(ctx, &mesh, param)
    }

}

pub struct GameState {
    pub player: Player,
    pub lasers: Vec<Laser>
}

/// All structs with this trait can draw themselves.
/// The mesh can be retrieved from a variable if it's constant.
pub trait Draw {
    /// Draw a mesh for this object
    fn mesh(&self, ctx: &mut Context) -> ggez::GameResult<graphics::Mesh>;
    
    /// Write draw params for this object
    fn draw_param(&self) -> graphics::DrawParam;
    
    /// Draw this object
    fn draw(&self, ctx: &mut Context) -> ggez::GameResult;
}

impl GameState {
    pub fn new(ctx: &Context) -> GameState {

        GameState {
            player: Player::new(ctx),
            lasers: Vec::new()
        }

    }
}

/// Will screenwrap anything
/// Wish I could just use [Wrapping](https://doc.rust-lang.org/std/num/struct.Wrapping.html) but it's in nightly.
pub fn screen_wrap((x, y): (&mut f32, &mut f32), ctx: &Context) -> () {

    let (ctx_width, ctx_height) = graphics::drawable_size(ctx);

    if *x > ctx_width {
        *x = *x - ctx_width;
    } else if *x < 0.0 {
        *x = *x + ctx_width;
    }

    if *y > ctx_height {
        *y = *y - ctx_height;
    } else if *y < 0.0 {
        *y = *y + ctx_height;
    }
}

/// Returns true when the object has exited the canvas
pub fn outside_window((x, y): (f32, f32), ctx: &Context) -> bool {
    
    let (ctx_width, ctx_height) = graphics::drawable_size(ctx);
    
    if x > ctx_width
    || x < 0.0
    || y > ctx_height
    || y < 0.0 {
        return true
    } {
        return false
    }

}
