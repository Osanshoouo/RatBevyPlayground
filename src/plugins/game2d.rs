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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("premium_skin.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let sphere = meshes.add(Mesh::from(shape::Icosphere {
	radius: 0.5,
	subdivisions: 4,
    }));

    let white = materials.add(StandardMaterial {
	base_color: Color::WHITE,
	unlit: true,
	..Default::default()
    });
    
    
    commands.spawn((
	sphere,
	
        Object,
        children![(
            Camera2d::default(),
            Transform::from_xy(4.0, 8.0).looking_at(Vec2::ZERO, Vec2::Y),
        )],
    )));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
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
