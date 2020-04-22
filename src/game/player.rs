use ggez::input::keyboard;
use ggez::{graphics, Context, GameResult};
use graphics::{Mesh, MeshBuilder, DrawParam};

use crate::game;
use game::movement::Movement;

/// The player, and
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub movement_force: f32,
    pub rotation_speed: f32,
    pub mov: Movement,
    pub fire_rate: f32
}

impl Player {

    /// Handle keyboard inputs and update the location of the Player accordingly
    pub fn update_movement(&mut self, ctx: &Context) {

        /* The current implementation does not allow external forces
        This could be easily achieved by having this call take additional params which set
        some force before movement calculation like gravity. This is (currently) not needed for this game.*/
        self.mov.force_x = 0.0;
        self.mov.force_y = 0.0;

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::A) {
            self.rotation -= self.rotation_speed
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::D) {
            self.rotation += self.rotation_speed
        }
        
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            self.mov.force_x += self.rotation.cos() * self.movement_force;
            self.mov.force_y += self.rotation.sin() * self.movement_force;
        }

        // Movement structs handles the physics
        self.mov.update();

        self.x += self.mov.speed_x;
        self.y += self.mov.speed_y;
    }

    pub fn debug_string(&self) -> String {
        format!(
            "Force x:           {}\n\
             Force y:           {}\n\
             Acceleration x:    {}\n\
             Acceleration y:    {}\n\
             Speed x:           {}\n\
             Speed y:           {}\n\
             Rotation speed:    {}\n",
            
            self.mov.force_x,
            self.mov.force_y,
            self.mov.acceleration_x,
            self.mov.acceleration_y,
            self.mov.speed_x,
            self.mov.speed_y,
            self.rotation_speed
        )
    }

}

impl game::Draw for Player {
    fn mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        MeshBuilder::new()
        .line(
            &[
                [0.0, 0.0],
                [self.height, -self.width / 2.0],
                [0.0, -self.width],
                [0.0,0.0]
            ],
            1.3,
            graphics::WHITE
        )?
        .build(ctx)
    }

    fn draw_param(&self) -> DrawParam {
        DrawParam::new()
            .dest([self.x, self.y])
            .offset([0.5 * self.height, 0.5 * -self.width])
            .rotation(self.rotation)
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let mesh = self.mesh(ctx)?;
        let param = self.draw_param();
        graphics::draw(ctx, &mesh, param)
    }

}
