use tetra::graphics::{Texture, Rectangle};
use tetra::math::Vec2;


#[derive(Debug, Clone)]
pub struct Entity {
    pub texture: Texture,
    pub coordinates: Coordinates,
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinates {
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub score: f32
}

impl Entity {
    pub fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        let coordinates = Coordinates{position, velocity: Vec2::zero(), score: 0.0};
        Entity {texture, coordinates}
    }

    pub fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        let coordinates = Coordinates{position, velocity, score: 0.0};
        Entity{
            texture,
            coordinates
        }
    }

    pub fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    pub fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.coordinates.position.x,
            self.coordinates.position.y,
            self.width(),
            self.height()
        )
    }

    pub fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.coordinates.position.x + (self.width() / 2.0),
            self.coordinates.position.y + (self.height() / 2.0),
        )
    }
}
