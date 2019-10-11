use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    core::{
        transform::Transform,
        math::{
            Point3, Vector3,
        },
        timing::Time,
    },
    window::ScreenDimensions,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, sprite::SpriteSheetHandle, Transparent},
    tiles::{Tile, TileMap, Map, CoordinateEncoder},
    ecs::prelude::*,
};

#[derive(Default, Clone)]
pub struct TestTile;
impl Tile for TestTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        Some(0)
    }
}

pub struct MapMovementSystem {
    rotate: bool,
    translate: bool,
    vector: Vector3<f32>,
}
impl Default for MapMovementSystem {
    fn default() -> Self {
        Self {
            rotate: true,
            translate: true,
            vector: Vector3::new(100.0, 0.0, 0.0),
        }
    }
}
impl<'s> System<'s> for MapMovementSystem {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, TileMap<TestTile>>,
    );

    fn run(&mut self, (time, mut transforms, tilemaps,): Self::SystemData) {
        if self.rotate {
            for (_, transform) in (&tilemaps, &mut transforms).join() {
                transform.rotate_2d(time.delta_seconds());
            }
        }
        if self.translate {
            for (_, transform) in (&tilemaps, &mut transforms).join() {
                transform.prepend_translation(self.vector * time.delta_seconds());
                if transform.translation().x > 500.0 {
                    self.vector = Vector3::new(-100.0, 0.0, 0.0);
                } else if transform.translation().x < -500.0 {
                    self.vector = Vector3::new(100.0, 0.0, 0.0);
                }
            }
        }
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

        init_sprite(world, &SpriteRender {
            sprite_sheet: sprite.clone(),
            sprite_number: 0,
        });

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
    transform.set_translation_xyz(640., 360., 0.);

    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .build();
}

fn init_map(world: &mut World, map: TileMap<TestTile>, dim: &ScreenDimensions) {
    let mut trans = Transform::default();
    // trans.set_translation_xyz(dim.width() * 0.5, dim.height() * 0.5, 0.0); // doesn't work because of a bug or smth
    trans.prepend_translation_x(dim.width());

    println!("Map Translation {:#?}", trans.translation());

    let _map_entity = world
        .create_entity()
        .with(map)
        .with(trans)
        .with(Transparent)
        .build();
}