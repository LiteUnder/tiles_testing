use amethyst::{
    prelude::*,
    ecs::prelude::*,
};

pub struct Player {
    velocity_x: f32,
    velocity_y: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            velocity_x: 0.0,
            velocity_y: 0.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Player>;
}

