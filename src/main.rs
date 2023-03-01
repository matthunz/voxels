use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use voxel::{player, BlockKind, Chunk, Selection};

fn main() {
    App::new()
        .insert_resource(Chunk::filled(BlockKind::Grass))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WireframePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    chunk: Res<Chunk>,
) {
    wireframe_config.global = true;

    /*
       let debug_material = materials.add(StandardMaterial {
           base_color_texture: Some(images.add(uv_debug_texture())),
           ..default()
       });
    */

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

    /*
    let mesh = meshes.add(shape::Cube::default().into());

    for block in chunk.iter() {
        match block.kind {
            BlockKind::Air => {}
            BlockKind::Grass => {
                commands.spawn((
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: debug_material.clone(),
                        transform: Transform::from_translation(block.position),
                        ..default()
                    },
                    Shape,
                ));
            }
        }
    }
    */

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

    // ground plane
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

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
