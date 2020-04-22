use ggez::{graphics, Context, GameResult};
use graphics::{Mesh, MeshBuilder, DrawParam};
use ggez::nalgebra::{Point2};

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
        
        self.player.update_movement(ctx);
        game::screen_wrap((&mut self.player.x, &mut self.player.y), &ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, graphics::BLACK);
       
        self.player.draw(ctx)?;

        let player_stats = self.player.debug_string();

        let debug_string = format!("Fps ({})\n{}", ggez::timer::fps(ctx), player_stats);
        graphics::draw(ctx, &(graphics::Text::new(debug_string)), graphics::DrawParam::default())?;


        graphics::present(ctx)
    }
}
