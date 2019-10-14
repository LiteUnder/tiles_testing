use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::Vector3, Transform},
    ecs::prelude::*,
    prelude::*,
    renderer::{
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        Camera, ImageFormat, Texture,
    },
    tiles::TileMap,
    window::ScreenDimensions,
};

use crate::game::{
    tiles::TestTile,
    components::Player,
};
pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone(); // may cause issues with resizing...
        let eye_sheet = load_sprite(world, "eye");
        let player_sheet = load_sprite(world, "player");

        world.register::<Player>();

        init_player(
            world,
            &SpriteRender {
                sprite_sheet: player_sheet,
                sprite_number: 0,
            },
            &dimensions,
        );

        let map = TileMap::<TestTile>::new(
            Vector3::new(20, 11, 1),
            Vector3::new(64, 64, 1),
            Some(eye_sheet),
        );

        TestTile::init_map(world, map);
        init_camera(world, &dimensions);
    }
}

fn init_camera(world: &mut World, dim: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dim.width() * 0.5, dim.height() * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(dim.width(), dim.height()))
        .with(transform)
        .build();
}

fn load_sprite(world: &mut World, name: &str) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();

    let tex_handle = {
        let tex_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(format!("sprites/{}.png", name), ImageFormat::default(), (), &tex_storage)
    };

    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("sprites/{}.ron", name),
        SpriteSheetFormat(tex_handle),
        (),
        &sheet_storage,
    )
}

fn init_player(world: &mut World, sprite: &SpriteRender, dim: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dim.width() * 0.5, dim.height() * 0.5, 0.1);
    transform.set_scale(Vector3::new(1.5, 1.5, 1.0));

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .with(Player::default())
        .build();
}