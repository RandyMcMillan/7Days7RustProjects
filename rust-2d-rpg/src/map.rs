use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Map {
    // Simplified map representation
    tiles: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Map {
        let tiles = vec![vec![true; 20]; 15];
        Map { tiles }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let tile_size = 32.0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile {
                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            (x as f32) * tile_size,
                            (y as f32) * tile_size,
                            tile_size,
                            tile_size,
                        ),
                        [0.5, 0.5, 0.5, 1.0].into(),
                    )?;
                    graphics::draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
                }
            }
        }
        Ok(())
    }
}
