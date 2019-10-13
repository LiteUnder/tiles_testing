use amethyst::ecs::prelude::*;
use amethyst_imgui;

#[derive(Default, Clone, Copy)]
pub struct ImguiWindow;
impl<'s> System<'s> for ImguiWindow {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        amethyst_imgui::with(|ui| {
            ui.show_demo_window(&mut true);
        });
    }
}
