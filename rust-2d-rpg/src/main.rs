mod enemy;
mod game;
mod map;
mod player;

use ggez::event::{self, EventHandler};
use ggez::{Context, ContextBuilder, GameResult};

fn main() -> GameResult {
    let cb = ContextBuilder::new("rust_2d_rpg", "YourName")
        .window_setup(ggez::conf::WindowSetup::default().title("2D RPG in Rust"));
    let (mut ctx, event_loop) = cb.build()?;
    let game = game::Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
