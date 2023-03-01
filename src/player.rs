use crate::{chunk::Block, BlockKind, Chunk, Selection};
use bevy::{
    input::mouse::{mouse_button_input_system, MouseMotion},
    prelude::*,
    window::CursorGrabMode,
};
use block_mesh::ndshape::Shape;
use std::{f32::consts::FRAC_PI_2, marker::PhantomData};

pub const DEFAULT_CAMERA_SENS: f32 = 0.005;

#[derive(Default, Component)]
pub struct PlayerController {
    yaw: f32,
    pitch: f32,
    cursor_locked: bool,
    looking_at: usize,
}

pub fn handle_player_mouse_move<S>(
    mut player_query: Query<(&mut PlayerController, &mut Transform)>,
    mut selection_query: Query<(&mut Transform, With<Selection>, Without<PlayerController>)>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut window: ResMut<Windows>,
    chunk: Res<Chunk<S>>,
) where
    S: Shape<3, Coord = u32> + Send + Sync + 'static,
{
    let (mut controller, mut transform) = player_query.single_mut();
    let mut delta = Vec2::ZERO;

    if controller.cursor_locked {
        for mouse_move in mouse_motion_event_reader.iter() {
            delta += mouse_move.delta;
        }
    }

    let first_win = window.get_primary_mut().unwrap();
    first_win.set_cursor_visibility(!controller.cursor_locked);
    first_win.set_cursor_grab_mode(if controller.cursor_locked {
        CursorGrabMode::Confined
    } else {
        CursorGrabMode::None
    });

    if delta == Vec2::ZERO {
        return;
    }

    let mut new_pitch = controller.pitch + delta.y * DEFAULT_CAMERA_SENS;
    let new_yaw = controller.yaw - delta.x * DEFAULT_CAMERA_SENS;

    new_pitch = new_pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

    controller.yaw = new_yaw;
    controller.pitch = new_pitch;

    transform.rotation =
        Quat::from_axis_angle(Vec3::Y, new_yaw) * Quat::from_axis_angle(-Vec3::X, new_pitch);

    if let Some(block) = raycast(&chunk, 10., &transform) {
        let (mut transform, (), ()) = selection_query.single_mut();
        transform.translation = block.position;

        controller.looking_at = block.index;
    }
}

pub fn handle_player_input(
    mut query: Query<(&mut PlayerController, &mut Transform)>,
    input: Res<Input<KeyCode>>,
) {
    let (mut controller, mut transform) = query.single_mut();

    if input.just_pressed(KeyCode::Escape) {
        controller.cursor_locked = !controller.cursor_locked;
    }

    let mut direction = Vec3::ZERO;

    let forward = transform.rotation.mul_vec3(Vec3::Z).normalize() * Vec3::new(1.0, 0., 1.0);
    let right = transform.rotation.mul_vec3(Vec3::X).normalize();

    let mut acceleration = 1.0f32;

    if input.pressed(KeyCode::W) {
        direction.z -= 1.0;
    }

    if input.pressed(KeyCode::S) {
        direction.z += 1.0;
    }

    if input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }

    if input.pressed(KeyCode::Space) {
        direction.y += 1.0;
    }

    if input.pressed(KeyCode::LShift) {
        direction.y -= 1.0;
    }

    if input.pressed(KeyCode::LControl) {
        acceleration *= 8.0;
    }

    if direction == Vec3::ZERO {
        return;
    }

    // hardcoding 0.10 as a factor for now to not go zoomin across the world.
    transform.translation += direction.x * right * acceleration
        + direction.z * forward * acceleration
        + direction.y * Vec3::Y * acceleration;
}

pub fn handle_player_click<S>(
    query: Query<&PlayerController>,
    mouse_button_input_events: Res<Input<MouseButton>>,
    mut chunk: ResMut<Chunk<S>>,
) where
    S: Shape<3, Coord = u32> + Send + Sync + 'static,
{
    let player = query.single();

    if mouse_button_input_events.just_pressed(MouseButton::Left) {
        chunk.blocks[player.looking_at] = BlockKind::Air;
    }

    if mouse_button_input_events.just_pressed(MouseButton::Right) {
        chunk.blocks[player.looking_at] = BlockKind::Grass;
    }
}

pub struct PlayerPlugin<S> {
    _marker: PhantomData<S>,
}

impl<S> Default for PlayerPlugin<S> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<S> Plugin for PlayerPlugin<S>
where
    S: Shape<3, Coord = u32> + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_system(mouse_button_input_system)
            .add_system(handle_player_mouse_move::<S>)
            .add_system(handle_player_input)
            .add_system(handle_player_click::<S>);
    }
}

fn raycast<S>(chunk: &Chunk<S>, max_length: f32, transform: &Transform) -> Option<Block>
where
    S: Shape<3, Coord = u32>,
{
    let direction = transform.forward().normalize();
    let origin = transform.translation;
    let step_size = 0.1;

    let mut current_pos = origin;
    while origin.distance_squared(current_pos) <= max_length.powi(2) {
        if let Some(block) = chunk.block(current_pos) {
            return Some(block);
        }
        current_pos += direction * step_size;
    }
    None
}
