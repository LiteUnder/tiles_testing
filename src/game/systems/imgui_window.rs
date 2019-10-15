use amethyst::ecs::prelude::*;
use amethyst_imgui;
use amethyst_imgui::imgui::*;

use crate::game::components::Player;
#[derive(Default, Clone, Copy)]
pub struct ImguiWindow;
impl<'s> System<'s> for ImguiWindow {
    type SystemData = (
        WriteStorage<'s, Player>,
    );

    fn run(&mut self, (mut players,): Self::SystemData) {
        for player in (&mut players).join() {
            amethyst_imgui::with(|ui| {
                Window::new(im_str!("Debug"))
                    .build(ui, || {
                        ui.text("Player speed");

                        Slider::<f32>::new(im_str!("Player speed"), 0.0..=1000.0).build(ui, &mut player.speed);
                    })
            });
        }
    }
}
