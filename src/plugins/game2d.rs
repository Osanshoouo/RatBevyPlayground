#![allow(unused)]
use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion, light::NotShadowCaster, prelude::*, reflect::TypePath,
    render::render_resource::AsBindGroup, shader::ShaderRef,
};

use bevy::{app::App, prelude::*};
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::TAU;

use crate::prelude::cube::Object;
use crate::prelude::player::Player;
use crate::prelude::player_camera::CameraSensitivity;

use super::camera2d;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    // Your game logic here
    // setup;
    // rotation;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));

    commands.spawn((Camera2d,));
}

pub(crate) fn rotation(time: Res<Time>, mut cubes: Query<(&mut Transform, &Object)>) {
    for (mut transform, cube) in &mut cubes {
        transform.translation.x = 0.0;
        transform.translation.y = 2.0;
        transform.translation.z = 0.0;
        transform.rotate_y(0.5 * TAU * time.delta_secs());
        transform.rotate_x(0.5 * TAU * time.delta_secs());
        transform.rotate_z(0.5 * TAU * time.delta_secs());
    }
}
