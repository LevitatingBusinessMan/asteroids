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
    game_state.spawn_asteroids(&mut ctx, 1);

    match ggez::event::run(&mut ctx, &mut event_loop, &mut game_state) {
        Ok(_)       => println!("Exited cleanly"),
        Err(err)    => println!("Error occured: {}", err)
    }

}

impl ggez::event::EventHandler for game::GameState {

    /* I should soon start to specify an update interval so physics match up */
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        self.ship.update_movement(ctx);

        // Wrap the ship around the screen
        game::screen_wrap((&mut self.ship.x, &mut self.ship.y), &ctx);

        'laserloop: for laser in &mut self.lasers {
    
            laser.update();
    
            for asteroid in &mut self.asteroids {

                asteroid.update();
    
                // Wrap asteroids around the screen
                game::screen_wrap((&mut asteroid.x, &mut asteroid.y), &ctx);
    
                if asteroid.in_hitbox((self.ship.x, self.ship.y)) {
                    
                    self.death();
                    break 'laserloop;

                } 

                if asteroid.in_hitbox((laser.x, laser.y)) {
                    println!("sick");
                    ()asteroid.split();
                }
    
            }
        
        }

        self.lasers.retain(|laser| !game::outside_window((laser.x, laser.y), ctx));

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Space)
        && self.ship.last_fire.elapsed() > Duration::from_secs_f32(1.0 / self.ship.fire_rate) {
            self.ship.shoot(&mut self.lasers);
            self.ship.last_fire = Instant::now();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, graphics::BLACK);

        let (ctx_width, ctx_height) = graphics::drawable_size(ctx);
        
        for laser in &mut self.lasers {
            laser.draw(ctx)?;
        }

        for asteroid in &mut self.asteroids {
            asteroid.draw(ctx)?;
        }
       
        self.ship.draw(ctx)?;

        // Debug stuff
        let ship_stats = self.ship.debug_string();
        let debug_string = format!(
            "Fps ({})\n\
            Lasers ({})\n\
            Asteroids ({})\n\
            {}",

            ggez::timer::fps(ctx),
            self.lasers.len(),
            self.asteroids.len(),
            ship_stats
        );
        graphics::draw(ctx, &(graphics::Text::new(debug_string)), graphics::DrawParam::default())?;


        graphics::present(ctx)
    }

    // TODO: add keydown functions instead

}
