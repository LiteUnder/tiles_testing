use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        math::{Point3, Vector3},
        Transform,
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        Camera, ImageFormat, Texture, Transparent,
    },
    tiles::{Map, Tile, TileMap},
    window::ScreenDimensions,
};

pub mod systems;
pub mod tiles;

use tiles::TestTile;
pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone(); // may cause issues with resizing...
        let spr_sheet = load_sprite(world);

        init_sprite(
            world,
            &SpriteRender {
                sprite_sheet: spr_sheet.clone(),
                sprite_number: 0,
            },
        );

        let map = TileMap::<TestTile>::new(
            Vector3::new(10, 10, 1),
            Vector3::new(64, 64, 1),
            Some(spr_sheet),
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

fn load_sprite(world: &mut World) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();

    let tex_handle = {
        let tex_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load("sprites/eye.png", ImageFormat::default(), (), &tex_storage)
    };

    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/eye.ron",
        SpriteSheetFormat(tex_handle),
        (),
        &sheet_storage,
    )
}

fn init_sprite(world: &mut World, sprite: &SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(640., 360., 0.);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();
}
