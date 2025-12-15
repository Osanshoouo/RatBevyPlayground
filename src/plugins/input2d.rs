#![allow(unused)]
use bevy::prelude::*;
use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion, light::NotShadowCaster, prelude::*, reflect::TypePath,
    render::render_resource::AsBindGroup, shader::ShaderRef,
};
use bevy::{input::keyboard::Key, prelude::*};
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::TAU;

// #[derive(Debug, Component)]
// pub struct Player;

// #[derive(Component)]
// struct Object;

use crate::prelude::cube::Object;
use crate::prelude::player::Player;
use crate::prelude::player_camera::CameraSensitivity;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, keyboard_input_system);
    // Your game logic here
    // keyboard_input_system;
    // move_player;
}

fn keyboard_input_system(
    mut cube: Query<(&mut Transform, &Object)>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_input: Res<ButtonInput<Key>>,
) {
    for (mut transform, cube) in &mut cube {
        if keyboard_input.pressed(KeyCode::KeyY) {
            transform.rotate_y(0.5 * TAU * time.delta_secs());
        }
        if keyboard_input.pressed(KeyCode::KeyO) {
            transform.rotate_x(0.5 * TAU * time.delta_secs());
        }
        if keyboard_input.pressed(KeyCode::KeyU) {
            transform.rotate_z(0.5 * TAU * time.delta_secs());
        }
        if keyboard_input.pressed(KeyCode::KeyL) {
            transform.translation.x += 10.0 * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.y += 10.0 * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyC) {
            transform.translation.z += 10.0 * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            transform.translation.x -= 10.0 * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyM) {
            transform.translation.y -= 10.0 * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.z -= 10.0 * time.delta_secs();
        }
        // else {
        //     transform.translation.x = 0.0;
        //     transform.translation.y = 0.0;
        //     transform.translation.z = 0.0;
        // }
    }
}

fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
