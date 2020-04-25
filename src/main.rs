// Important stuff to do
// Think about queueing the draw stuff
// Use key events?
// Make physics loop on a consistent timeout
// Fix ship hit detection (thus fixing its origin point)

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
    game_state.spawn_asteroids(&mut ctx, 5);

    match ggez::event::run(&mut ctx, &mut event_loop, &mut game_state) {
        Ok(_)       => println!("Exited cleanly"),
        Err(err)    => println!("Error occured: {}", err)
    }

}

impl ggez::event::EventHandler for game::GameState {

    /* I should soon start to specify an update interval so physics match up */
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        // Undead yourself
        if self.dead {
            if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Space) {
                self.dead = false;
                self.spawn_asteroids(ctx, 5);
            } else {
                return Ok(())
            }
        }

        self.ship.update_movement(ctx);

        // Wrap the ship around the screen
        let mut asteroids_to_add = Vec::new();
        let mut lasers_to_remove = Vec::new();

        game::screen_wrap((&mut self.ship.x, &mut self.ship.y), &ctx);

        for asteroid in &mut self.asteroids {
            asteroid.update();

            if asteroid.in_hitbox((self.ship.x, self.ship.y)) {
                    
                    self.death(&ctx);
                    break;

            } 

    
            // Wrap asteroids around the screen
            game::screen_wrap((&mut asteroid.x, &mut asteroid.y), &ctx);
        }

        for (laser_index, laser) in &mut self.lasers.iter_mut().enumerate() {
    
            laser.update();
    
            'asteroidloop: for (asteroid_index, asteroid) in &mut self.asteroids.iter_mut().enumerate() {
    
                if asteroid.in_hitbox((laser.x, laser.y)) {
                    let output = asteroid.split();
                    
                    if let Some(newAsteroids) = &output {
                        asteroids_to_add.extend_from_slice(newAsteroids);
                    }

                    self.points += 1;

                    // Remove asteroid
                    self.asteroids.remove(asteroid_index);
                    
                    //Remove laser (later)
                    lasers_to_remove.push(laser_index);

                    break 'asteroidloop;

                }
    
            }
        
        }

        for index in lasers_to_remove {
            self.lasers.remove(index);
        }

        // add new asteroids
        self.asteroids.append(&mut asteroids_to_add);

        if self.asteroids.len() == 0 {
            self.spawn_asteroids(ctx, 5);
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
        
        if self.dead {

            // To make it center align I just made 2 messages

            let text1 = graphics::Text::new("YOU SEEM TO HAVE DIED");
            let text2 = graphics::Text::new("Space to restart :)");
            
            let width1 = text1.width(ctx);
            let width2 = text2.width(ctx);
            let height = text1.height(ctx);

            graphics::draw(
                ctx,
                &text1,
                graphics::DrawParam::default()
                    .dest([
                        (ctx_width - width1 as f32) / 2.0,
                        (ctx_height - height as f32) / 2.0]
                    )
            )?;

            graphics::draw(
                ctx,
                &text2,
                graphics::DrawParam::default()
                    .dest([
                        (ctx_width - width2 as f32) / 2.0,
                        (ctx_height - height as f32) / 2.0 + height as f32]
                    )
            )?;
        }

        // This should actually be queued but whatever
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
