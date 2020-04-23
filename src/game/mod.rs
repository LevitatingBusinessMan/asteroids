use ggez::{graphics, Context};

use std::vec::Vec;

mod movement;

mod objects;
use objects::{Ship, Laser, Asteroid};

/* In the future I should make general things like
GameObject (Struct)
and GameObjectCollection (Vec)
sharing specifc traits and enum options etc 
Like an enum if they should wrap or destruct */

pub struct GameState {
    pub ship: Ship,
    pub lasers: Vec<Laser>,
    pub asteroids: Vec<Asteroid>
}

/// All structs with this trait can draw themselves.
pub trait Draw {
    /// Draw a mesh for this object
    fn mesh(&self, ctx: &mut Context) -> ggez::GameResult<graphics::Mesh>;
    
    /// Write draw params for this object
    fn draw_param(&self) -> graphics::DrawParam;
    
    /// Draw this object
    fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        let mesh = self.mesh(ctx)?;
        let param = self.draw_param();
        graphics::draw(ctx, &mesh, param)
    }
}

impl GameState {
    pub fn new(ctx: &Context) -> GameState {

        GameState {
            ship: Ship::new(ctx),
            lasers: Vec::new(),
            asteroids: Vec::new()
        }

    }

    pub fn spawn_asteroids(& mut self, ctx: &mut Context, amount: u32) {
        for _ in 0..amount {
            self.asteroids.push(Asteroid::new(self.ship.x, self.ship.y, ctx))
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

pub fn random_place(ctx: &Context) -> (f32, f32){
    let (ctx_width, ctx_height) = graphics::drawable_size(ctx);

    let x = rand::random::<f32>() * ctx_width;
    let y = rand::random::<f32>() * ctx_height;

    (x, y)

}
