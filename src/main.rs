use ggez::{graphics, Context, GameResult};
use ggez::input::keyboard;

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
    pub fn update(&mut self) {
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
    walk_force: f32,
    mov: Movement
}

struct GameState {
    player: Player
}

impl GameState {
    pub fn new(ctx: &Context) -> GameState {
        
        let (ctx_width, ctx_height) = graphics::drawable_size(ctx);

        let (player_width, player_height) = (10.0, 10.0);

        GameState {
            player: Player {
                width: player_width,
                height: player_height,
                x: ctx_width / 2.0 - player_width,
                y: ctx_height / 2.0 - player_height,
                walk_force: 5.0,
                mov: Movement {
                    drag: 0.5,
                    mass: 10.0,
                    speed_x: 0.0,
                    speed_y: 0.0,
                    force_x: 0.0,
                    force_y: 0.0,
                    accellartion_x: 0.0,
                    accellartion_y: 0.0 
                }
            }
        }

    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        self.player.mov.force_x = 0.0;
        self.player.mov.force_y = 0.0;

        // Set forces by player from walking
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::A) {
            self.player.mov.force_x -= self.player.walk_force
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::D) {
            self.player.mov.force_x += self.player.walk_force
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            self.player.mov.force_y -= self.player.walk_force
        }
        
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S) {
            self.player.mov.force_y += self.player.walk_force
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

        if self.player.y > ctx_width {
            self.player.y = self.player.y - ctx_height;
        } else if self.player.x < 0.0 {
            self.player.y = self.player.y + ctx_height;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
       
        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.player.x, self.player.y, self.player.width, self.player.height),
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &player_mesh, graphics::DrawParam::default())?;

        let player_stats = format!( 
            "Force x:           {}\n\
             Force y:           {}\n\
             Acceleration x:    {}\n\
             Acceleration y:    {}\n\
             Speed x:           {}\n\
             Speed y:           {}\n",
            
            self.player.mov.force_x,
            self.player.mov.force_y,
            self.player.mov.accellartion_x,
            self.player.mov.accellartion_y,
            self.player.mov.speed_x,
            self.player.mov.speed_y
        );

        graphics::draw(ctx, &(graphics::Text::new(player_stats)), graphics::DrawParam::default())?;


        graphics::present(ctx)
    }
}
