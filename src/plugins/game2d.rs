#![allow(unused)]
use bevy::math::VectorSpace;
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

#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Debug)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.)
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, simulate);
    app.add_systems(Update, sync_transforms);
    // Your game logic here
    // setup;
    // rotation;
}

pub const DELTA_TIME: f32 = 1. / 60.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        PrevPos(Vec2::ZERO - Vec2::new(5., 0.) * DELTA_TIME),
        Pos(Vec2::ZERO),
        Mass(1.),
    ));
    commands.spawn((Camera2d,));
}

pub fn simulate(mut query: Query<(&mut Pos, &mut PrevPos, &Mass)>) {
    for (mut pos, mut prev_pos, mass) in query.iter_mut() {
        let gravity = Vec2::new(0., -9.81);
        let grivitational_force = mass.0 * gravity;
        let external_forces = grivitational_force;
        let velocity = (pos.0 - prev_pos.0) / DELTA_TIME + DELTA_TIME * external_forces / mass.0;

        prev_pos.0 = pos.0;
        pos.0 = pos.0 + velocity * DELTA_TIME;
    }
}

pub fn sync_transforms(mut query: Query<(&mut bevy::transform::components::Transform, &Pos)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
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
