use ggez::input::keyboard;
use ggez::{graphics, Context, GameResult};
use graphics::{Mesh, MeshBuilder, DrawParam};
use ggez::nalgebra::Point2;

use crate::game;
use game::movement::Movement;
use game::Draw;

//#region Ship
/// The Ship.\
/// Width and Height is sort of switched here.\
/// This is because the mesh is made to face the right but then rotated upwards.\
/// I thought it would make more sense like this but it kind of didn't but whateer who cares.\
pub struct Ship {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub mov: Movement,
   
    /// Current rotation in radials
    pub rotation: f32, 
   
    /// Force to add to the movement calculation when using rocket
    pub movement_force: f32, 
   
    /// Speed of rotation in radials per tick 
    pub rotation_speed: f32,
   
    /// How many time the ship can fire a laser per second
    pub fire_rate: f32,
   
    /// Time the ship fired for the last time
    pub last_fire: std::time::Instant,
   
    /// If the ship is currently using it's rocket
    pub moving: bool 
}

impl Ship {

    pub fn new(ctx: &Context) -> Ship {
        
        let (ctx_width, ctx_height) = graphics::drawable_size(ctx);

        let ship_width = 18.0;
        let ship_height = 20.0;

        Ship {
            width: ship_width,
            height: ship_height,
            x: (ctx_width - ship_width)/ 2.0,
            y: (ctx_height- ship_height) / 2.0,
            rotation: (3.0 / 2.0) * std::f32::consts::PI, // Start facing up
            movement_force: 5.0,
            rotation_speed: 0.08,
            mov: Movement::new(0.3, 10.0),
            fire_rate: 5.0, 
            last_fire: std::time::Instant::now(),
            moving: false
        }
    }

    /// Handle keyboard inputs and update the location of the Ship accordingly
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


    /// Add a laser to the gamestate appearing from the ship
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

impl game::Draw for Ship {
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

}
//#endregion

//#region Laser
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
        MeshBuilder::new()
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

    fn draw_param(&self) -> DrawParam {
        DrawParam::new()
            .dest([self.x, self.y])
            .offset([0.5 * self.width, 0.0])
            .rotation(self.rotation)
    }

}
//#endregion

//#region Asteroid

/// The 3 different asteroid sizes
#[derive(Debug)]
enum AsteroidSize {
    Big,
    Medium,
    Small
}

/// Factor to multiple mesh with
const ASTEROID_BIG: f32 = 40.0;
/// Factor to multiple mesh with
const ASTEROID_MEDIUM: f32 = 30.0;
/// Factor to multiple mesh with
const ASTEROID_SMALL: f32  = 20.0;

pub struct Asteroid {
    pub x: f32,
    pub y: f32,
    rotation: f32,
    rotation_speed: f32,
    speed: f32,
    size: AsteroidSize,
    mirrored: bool,

    /// Index for the asteroid_mashes var
    mesh: usize

}

const ASTEROID_MAX_MOVEMENT_SPEED: f32 = 0.0;
const ASTEROID_MAX_ROTATION_SPEED: f32 = 0.0;

/// The width/height of the safezone of the ship.\
/// Asteroids do not spawn here
const SHIP_SAFEZONE: f32 = 300.0;

/// Array of different random meshes for the asteroids
const ASTEROID_MESHES: [fn(f32) -> [[f32;2];13];1] = [
    |size| [
        [0.0 *size, 0.0 *size],
        [1.0 *size, 0.0 *size],
        [2.5 *size, 1.0 *size],
        [2.5 *size, 1.3 *size],
        [1.5 *size, 1.7 *size],
        [2.4 *size, 1.9 *size],
        [1.5 *size, 2.8 *size],
        [0.9 *size, 2.6 *size],
        [0.4 *size, 2.4 *size],
        [-0.3*size, 1.2 *size],
        [-0.1*size, 0.8 *size],
        [0.3 *size, 1.0 *size],
        [0.0 *size, 0.0 *size]
    ]
];


impl Asteroid {
    pub fn new(ship_x: f32, ship_y: f32, ctx: &mut Context) -> Asteroid {


        let (mut x, mut y);

        loop {

            // Can't shadow via pattern :(
            let (x_, y_) = game::random_place(ctx);
            x = x_;
            y = y_;

            // Break out when the coords are not in a safezone
            if ship_x - x < SHIP_SAFEZONE / 2.0 || ship_x - x > -(SHIP_SAFEZONE / 2.0)
            || ship_y - y > SHIP_SAFEZONE / 2.0 || ship_y - y < -(SHIP_SAFEZONE / 2.0)
            {
                break;
            }
        }

        let size;

        match (rand::random::<f32>() * 3.0 + 1.0).floor() as u8 {
            1 => size = AsteroidSize::Small,
            2 => size = AsteroidSize::Medium,
            3 => size = AsteroidSize::Big,
            _ => size = AsteroidSize::Small
        }

        let mirrored = match rand::random::<f32>().round() as u8 {
            1 => false,
            2 => true,
            _ => true
        };

        let speed = rand::random::<f32>() * ASTEROID_MAX_MOVEMENT_SPEED;
        let rotation_speed = rand::random::<f32>() * ASTEROID_MAX_ROTATION_SPEED;
        let rotation = rand::random::<f32>() * (2.0 * std::f32::consts::PI);

        let mesh = (rand::random::<f32>() * ASTEROID_MESHES.len() as f32).floor() as usize;

        // Asteroid go brrr
        Asteroid {
            x,
            y,
            size,
            speed,
            rotation_speed,
            rotation,
            mirrored,
            mesh
        }
    }

    pub fn update(&mut self) {
        self.x += self.rotation.cos() * self.speed;
        self.y += self.rotation.sin() * self.speed;

        self.rotation += self.rotation_speed;

    }

}

impl Draw for Asteroid {
    
    fn mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {

        let size;
        match &self.size {
            AsteroidSize::Big => size = ASTEROID_BIG,
            AsteroidSize::Medium => size = ASTEROID_MEDIUM,
            AsteroidSize::Small => size = ASTEROID_SMALL
        }

        Mesh
        ::new_line(
            ctx,
            &ASTEROID_MESHES[self.mesh](size),
            1.0,
            graphics::WHITE
        )
    }

    fn draw_param(&self) -> DrawParam {

        let size;
        match &self.size {
            AsteroidSize::Big => size = ASTEROID_BIG,
            AsteroidSize::Medium => size = ASTEROID_MEDIUM,
            AsteroidSize::Small => size = ASTEROID_SMALL
        }

        let mut param = DrawParam::new()
            .dest([self.x, self.y])
            .offset([0.5 * size, 0.5 * -size])
            .rotation(self.rotation);
        
        if self.mirrored {
            param = param.scale([-1.0, 1.0]);
        }
        
        param

    }

}

//#endregion Asteroid
