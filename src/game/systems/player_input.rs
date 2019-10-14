use amethyst::{
    prelude::*,
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::game::components::Player;

#[derive(Default)]
pub struct PlayerInput;
impl<'s> System<'s> for PlayerInput {
    type SystemData = (
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut players, input): Self::SystemData) {
        for player in (&mut players).join() {

            player.velocity_x = input.axis_value("horizontal").unwrap_or(0.0);
            player.velocity_y = input.axis_value("vertical").unwrap_or(0.0);

            println!("x: {}\ny: {}", player.velocity_x, player.velocity_y);
        }
    }
}
