use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::RenderTiles2D,
    utils::application_root_dir,
};
use amethyst_imgui::RenderImgui;

mod game;
use game::states::MainState;
use game::systems::ImguiWindow;
use game::tiles::TestTile;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let res_dir = app_root.join("res");
    let display_config_path = res_dir.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::default())?
        .with(ImguiWindow::default(), "ImguiWindow", &["input_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<TestTile>::default())
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?;

    let mut game = Application::new(res_dir, MainState, game_data)?;
    game.run();

    Ok(())
}
