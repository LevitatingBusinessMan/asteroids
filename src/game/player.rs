use ggez::input::keyboard;
use ggez::{graphics, Context, GameResult};
use graphics::{Mesh, MeshBuilder, DrawParam};

use crate::game;
use game::movement::Movement;

/// The player.
/// Width and Height is sort of switched here.
/// This is because the mesh is made to face the right but then rotated upwards.
/// I thought it would make more sense like this but it kind of didn't but whateer who cares.
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub movement_force: f32,
    pub rotation_speed: f32,
    pub mov: Movement,
    pub fire_rate: f32,
    pub last_fire: std::time::Instant,
    pub moving: bool
}

impl Player {

    pub fn new(ctx: &Context) -> Player {
        
        let (ctx_width, ctx_height) = graphics::drawable_size(ctx);

        let player_width = 18.0;
        let player_height = 20.0;

        Player {
            width: player_width,
            height: player_height,
            x: (ctx_width - player_width)/ 2.0,
            y: (ctx_height- player_height) / 2.0,
            rotation: (3.0 / 2.0) * std::f32::consts::PI, // Start facing up
            movement_force: 5.0,
            rotation_speed: 0.08, /// Speed of rotation in radials per tick
            mov: Movement::new(0.3, 10.0),
            fire_rate: 5.0, /// How many time the ship can fire a laser per second
            last_fire: std::time::Instant::now(), /// Time the ship fired for the last time
            moving: false
        }
    }

    /// Handle keyboard inputs and update the location of the Player accordingly
    pub fn update_movement(&mut self, ctx: &Context) {

        /* The current implementation does not allow external forces
        This could be easily achieved by having this call take additional params which set
        some force before movement calculation like gravity. This is (currently) not needed for this game.*/
        self.mov.force_x = 0.0;
        self.mov.force_y = 0.0;

        self.moving = false;

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::A) {
            self.rotation -= self.rotation_speed;
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::D) {
            self.rotation += self.rotation_speed;
        }
        
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            self.mov.force_x += self.rotation.cos() * self.movement_force;
            self.mov.force_y += self.rotation.sin() * self.movement_force;
            self.moving = true;
        }

        // Movement structs handles the physics
        self.mov.update();

        self.x += self.mov.speed_x;
        self.y += self.mov.speed_y;
    }


    /// Add a laser to the gamestate appearing from the player
    pub fn shoot(&self, lasers:  &mut Vec<game::Laser>) {
        lasers.push(game::Laser::new(
            self.x + self.height /2.0,
            self.y - self.width / 2.0,
            self.rotation)
        );
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
        let mut mesh = MeshBuilder::new();
        
        // Could be a polygon as well
        mesh.line(
            &[
                [0.0, 0.0],
                [self.height, -self.width / 2.0],
                [0.0, -self.width],
                [0.0,0.0]
            ],
            1.3,
            graphics::WHITE
        )?;

        // Draw fire behind rocket
        if self.moving {
            mesh.line(
                &[
                    [ - 0.0, - 0.1 * self.width],
                    [ - 0.3 * self.height, - 0.233 * self.width],
                    [ - 0.2 * self.height, - 0.366 * self.width],
                    [ - 0.6 * self.height, - 0.5 * self.width],
                    [ - 0.2 * self.height, - 0.633 * self.width],
                    [ - 0.3 * self.height, - 0.766 * self.width],
                    [ - 0.0 * self.height, - 0.9 * self.width]
                ],
                1.3,
                graphics::WHITE
            )?;
        }

        mesh.build(ctx)
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
