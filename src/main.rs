use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};
use block_mesh::ndshape::ConstShape3u32;
use voxel::{player, BlockKind, Selection};

const CHUNK_SIZE: u32 = 4;
const CHUNK_DEPTH: u32 = 8;

type ChunkShape = ConstShape3u32<CHUNK_SIZE, CHUNK_SIZE, CHUNK_DEPTH>;

type Chunk = voxel::chunk::Chunk<ChunkShape>;

fn main() {
    let mut chunk = Chunk::filled(BlockKind::Air, ChunkShape {});
    *chunk.block_mut(Vec3::new(2., 2., 2.)).unwrap() = BlockKind::Grass;

    App::new()
        .insert_resource(chunk)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WireframePlugin)
        .add_plugin(player::PlayerPlugin::<ChunkShape>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    chunk: Res<Chunk>,
) {
    wireframe_config.global = true;

    let debug_material = materials.add(StandardMaterial::from(Color::BLUE));

    let mesh = meshes.add(chunk.mesh());
    commands.spawn((
        PbrBundle {
            mesh: mesh,
            material: debug_material,
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        },
        Shape,
    ));

    let selection_mesh = meshes.add(shape::Cube::new(1.1).into());
    let selection_material = materials.add(StandardMaterial::from(Color::RED));
    commands.spawn((
        PbrBundle {
            mesh: selection_mesh,
            material: selection_material,
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        },
        Selection,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 50.0 }.into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6., 12.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        })
        .insert(player::PlayerController::default());
}
