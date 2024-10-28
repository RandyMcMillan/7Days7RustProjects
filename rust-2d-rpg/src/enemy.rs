use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub struct Enemy {
    position: na::Point2<f32>,
}

impl Enemy {
    pub fn new(ctx: &mut Context) -> GameResult<Enemy> {
        Ok(Enemy {
            position: na::Point2::new(200.0, 200.0),
        })
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.position.x, self.position.y, 32.0, 32.0),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))
    }

    // Add methods for enemy movement, AI, etc.
}
