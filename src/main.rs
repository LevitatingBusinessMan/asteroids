use ggez::{graphics, Context, GameResult};
use graphics::{Mesh, MeshBuilder, DrawParam};
use ggez::nalgebra::{Point2};
use ggez::input::keyboard;
use std::time::{Instant, Duration};

mod game;

//Bring trait in scope because that's necessary for some reason
use game::Draw;

fn main() {
    
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Asteroids", "LevitatingBusinessMan")
    .build()
    .unwrap();

    let mut game_state = game::GameState::new(&ctx);

    match ggez::event::run(&mut ctx, &mut event_loop, &mut game_state) {
        Ok(_)       => println!("Exited cleanly"),
        Err(err)    => println!("Error occured: {}", err)
    }
}

impl ggez::event::EventHandler for game::GameState {

    /* I should soon start to specify an update interval so physics match up */
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        let now = Instant::now();

        self.player.update_movement(ctx);

        // Wrap the player around the screen
        game::screen_wrap((&mut self.player.x, &mut self.player.y), &ctx);

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Space)
        && self.player.last_fire.elapsed() > Duration::from_secs_f32(1.0 / self.player.fire_rate) {
            self.player.shoot(&mut self.lasers);
            self.player.last_fire = Instant::now();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, graphics::BLACK);

        let (ctx_width, ctx_height) = graphics::drawable_size(ctx);
        
        self.lasers.retain(|laser| !game::outside_window((laser.x, laser.y), ctx));
        
        for laser in &mut self.lasers {
            laser.update();
            laser.draw(ctx)?;
        }
       
        self.player.draw(ctx)?;

        // Debug stuff
        let player_stats = self.player.debug_string();
        let debug_string = format!(
            "Fps ({})\n\
            Lasers ({})\n\
            {}",

            ggez::timer::fps(ctx),
            self.lasers.len(),
            player_stats
        );
        graphics::draw(ctx, &(graphics::Text::new(debug_string)), graphics::DrawParam::default())?;


        graphics::present(ctx)
    }
}
