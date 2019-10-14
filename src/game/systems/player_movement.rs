use amethyst::{
    ecs::prelude::*,
    core::{
        Transform,
        Time,
    },
};

use crate::game::components::Player;

#[derive(Default)]
pub struct PlayerMovement;
impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, players, time): Self::SystemData) {
        for (transform, player) in (&mut transforms, &players).join() {
            let scaled_x = transform.translation().x + (player.velocity_x * time.delta_seconds());
            let scaled_y = transform.translation().y + (player.velocity_y * time.delta_seconds());
            
            transform.set_translation_xyz(scaled_x, scaled_y, 0.1);
        }
    }
}