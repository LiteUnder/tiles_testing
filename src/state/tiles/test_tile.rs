use amethyst::{
    core::{math::Point3, Transform},
    prelude::*,
    renderer::Transparent,
    tiles::{Map, Tile, TileMap},
};

#[derive(Default, Clone)]
pub struct TestTile;
impl Tile for TestTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        Some(0)
    }
}

impl TestTile {
    pub fn init_map(world: &mut World, map: TileMap<TestTile>) {
        let mut transform = Transform::default();
        // println!("Map Translation {:#?}", transform.translation());
        // transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
        transform.set_translation_x(
            (map.dimensions().x * map.tile_dimensions().x) as f32 * 0.5
                + (map.tile_dimensions().x as f32 * 0.5),
        );
        transform.set_translation_y(
            (map.dimensions().y * map.tile_dimensions().y) as f32 * 0.5
                - (map.tile_dimensions().y as f32 * 0.5),
        );

        let _map_entity = world
            .create_entity()
            .with(map)
            .with(transform)
            .with(Transparent)
            .build();
    }
}
