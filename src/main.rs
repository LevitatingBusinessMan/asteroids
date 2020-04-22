use ggez::{graphics, Context, GameResult};
use graphics::{Mesh, MeshBuilder};
use ggez::input::keyboard;
use ggez::nalgebra::{Point2};

fn main() {
    
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Asteroids", "LevitatingBusinessMan")
    .build()
    .unwrap();

    let mut game_state = GameState::new(&ctx);

    match ggez::event::run(&mut ctx, &mut event_loop, &mut game_state) {
        Ok(_)       => println!("Exited cleanly"),
        Err(err)    => println!("Error occured: {}", err)
    }

}

struct Movement {
    drag: f32,
    mass: f32,
    speed_x: f32,
    speed_y: f32,
    force_x: f32,
    force_y: f32,
    accellartion_x: f32,
    accellartion_y: f32
}

impl Movement {
    pub fn new(drag: f32, mass: f32) -> Movement {
        Movement {
            drag: drag,
            mass: mass,
            speed_x: 0.0,
            speed_y: 0.0,
            force_x: 0.0,
            force_y: 0.0,
            accellartion_x: 0.0,
            accellartion_y: 0.0
        }
    }

    pub fn update(&mut self) {
        // Movement
        self.accellartion_x = (self.force_x - self.drag * self.speed_x) / self.mass;
        self.accellartion_y = (self.force_y - self.drag * self.speed_y) / self.mass;

        self.speed_x +=  self.accellartion_x;
        self.speed_y +=  self.accellartion_y;

    }
}

struct Player {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    rotation: f32,
    movement_force: f32,
    rotation_speed: f32,
    mov: Movement
}

struct GameState {
    player: Player
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
                mov: Movement::new(0.3, 10.0)
            }
        }

    }
}

impl ggez::event::EventHandler for GameState {

    /* I should soon start to specify an update interval so physics match up */
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        self.player.mov.force_x = 0.0;
        self.player.mov.force_y = 0.0;


        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::A) {
            self.player.rotation -= self.player.rotation_speed
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::D) {
            self.player.rotation += self.player.rotation_speed
        }
        
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            self.player.mov.force_x += self.player.rotation.cos() * self.player.movement_force;
            self.player.mov.force_y += self.player.rotation.sin() * self.player.movement_force;
        }

        // Movement structs handles the physics
        self.player.mov.update();

        self.player.x += self.player.mov.speed_x;
        self.player.y += self.player.mov.speed_y;

        let (ctx_width, ctx_height) = graphics::drawable_size(ctx);

        // Handle player screen wrapping.
        // Wish I could just use https://doc.rust-lang.org/std/num/struct.Wrapping.html
        if self.player.x > ctx_width {
            self.player.x = self.player.x - ctx_width;
        } else if self.player.x < 0.0 {
            self.player.x = self.player.x + ctx_width;
        }

        if self.player.y > ctx_height {
            self.player.y = self.player.y - ctx_height;
        } else if self.player.y < 0.0 {
            self.player.y = self.player.y + ctx_height;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, graphics::BLACK);
       
        let player_mesh = MeshBuilder::new()
        .line(
            &[
                [0.0, 0.0],
                [self.player.height, -self.player.width / 2.0],
                [0.0, -self.player.width],
                [0.0,0.0]
            ],
            1.3,
            graphics::WHITE
        )?
        .build(ctx)?;

        graphics::draw(
            ctx,
            &player_mesh,
            graphics::DrawParam::new()
                .dest([self.player.x, self.player.y])
                .offset([0.5 * self.player.height, 0.5 * -self.player.width])
                .rotation(self.player.rotation)
        )?;

        //self.player.rotation

        let player_stats = format!( 
            "Fps:               {}\n\
             Force x:           {}\n\
             Force y:           {}\n\
             Acceleration x:    {}\n\
             Acceleration y:    {}\n\
             Speed x:           {}\n\
             Speed y:           {}\n\
             Rotation speed:    {}\n",
            
            ggez::timer::fps(ctx),
            self.player.mov.force_x,
            self.player.mov.force_y,
            self.player.mov.accellartion_x,
            self.player.mov.accellartion_y,
            self.player.mov.speed_x,
            self.player.mov.speed_y,
            self.player.rotation_speed
        );

        graphics::draw(ctx, &(graphics::Text::new(player_stats)), graphics::DrawParam::default())?;


        graphics::present(ctx)
    }
}
