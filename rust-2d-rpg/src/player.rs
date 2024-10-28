use ggez::graphics::{self, Rect};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub struct Player {
    position: na::Point2<f32>,
}

impl Player {
    pub fn new(_ctx: &mut Context) -> GameResult<Player> {
        Ok(Player {
            position: na::Point2::new(100.0, 100.0),
        })
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(self.position.x, self.position.y, 32.0, 32.0),
            graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))
    }

    // Add methods for movement, collision detection, etc.
}
