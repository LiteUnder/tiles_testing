use amethyst::{
    ecs::prelude::*,
};

pub struct Player {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            velocity_x: 0.0,
            velocity_y: 0.0,
            speed: 200.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Player>;
}

