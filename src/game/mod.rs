use ggez::{graphics, Context};

use std::vec::Vec;

mod movement;

mod objects;
use objects::{Ship, Laser};

pub struct GameState {
    pub ship: Ship,
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
            ship: Ship::new(ctx),
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
