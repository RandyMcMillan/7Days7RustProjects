use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Game {
    player: player::Player,
    map: map::Map,
    enemies: Vec<enemy::Enemy>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let player = player::Player::new(ctx)?;
        let map = map::Map::new();
        let enemies = vec![enemy::Enemy::new(ctx)?];
        Ok(Game {
            player,
            map,
            enemies,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update logic for player movement, enemy behavior, etc.
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.map.draw(ctx)?;
        self.player.draw(ctx)?;
        for enemy in &self.enemies {
            enemy.draw(ctx)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}
