use ggez::{graphics, Context};

use std::vec::Vec;

mod movement;
use movement::Movement;

mod player;
use player::Player;

/* pub struct Laser {
    
} */

pub struct GameState {
    pub player: Player/* ,
    lasers: Vec<Laser>*/
}

/// All structs with this trait can draw themselves
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
        
        let (ctx_width, ctx_height) = graphics::drawable_size(ctx);

        let player_width = 18.0;
        let player_height = 20.0;

        GameState {
            player: Player {
                width: player_width,
                height: player_height,
                x: (ctx_width - player_width)/ 2.0,
                y: (ctx_height- player_height) / 2.0,
                rotation: (3.0 / 2.0) * std::f32::consts::PI, // Start facing up
                movement_force: 5.0,
                rotation_speed: 0.08,
                mov: Movement::new(0.3, 10.0),
                fire_rate: 0.3,
                moving: false
            }
        }

    }
}

/// Will screenwrap anything
/// Wish I could just use https://doc.rust-lang.org/std/num/struct.Wrapping.html but it's in nightly
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
