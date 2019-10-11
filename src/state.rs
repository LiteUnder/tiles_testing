use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    core::{
        transform::Transform,
        math::{
            Point3, Vector3,
        }
    },
    window::ScreenDimensions,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, sprite::SpriteSheetHandle},
    tiles::{Tile, TileMap},
};

#[derive(Default, Clone)]
pub struct TestTile;
impl Tile for TestTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        Some(0)
    }
}

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone(); // may cause issues with resizing...

        let sprite = load_sprite(world);

        let map = TileMap::<TestTile>::new(
            Vector3::new(10, 10, 1),
            Vector3::new(64, 64, 1),
            Some(sprite.clone())
        );
        
        init_map(world, map, &dimensions);

        init_camera(world, &dimensions);
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprite(world: &mut World) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();

    let tex_handle = {
        let tex_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/eye.png",
            ImageFormat::default(),
            (),
            &tex_storage,
        )
    };

    let sheet_handle = {
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/eye.ron",
            SpriteSheetFormat(tex_handle),
            (),
            &sheet_storage,
        )
    };

    sheet_handle
}

fn init_sprite(world: &mut World, sprite: &SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(32.0, 720.0 - 32.0, 0.0);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();
}

fn init_map(world: &mut World, map: TileMap<TestTile>, dim: &ScreenDimensions) {
    let mut trans = Transform::default();
    trans.set_translation_xyz(dim.width() * 0.5, dim.height() * 0.5, 0.0);

    world
        .create_entity()
        .with(trans)
        .with(map)
        .build();
}