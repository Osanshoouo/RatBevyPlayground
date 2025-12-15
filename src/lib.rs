#![allow(unused_imports)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*};

use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion, light::NotShadowCaster, prelude::*, reflect::TypePath,
    render::render_resource::AsBindGroup, shader::ShaderRef,
};

mod components;
mod plugins;

/// Use this module instead of importing the `components`, `plugins`, `resources`, and `utils`
/// modules directly.
mod prelude {
    pub use super::*;
    pub use {components::*, plugins::*};
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            plugins::camera::plugin,
            plugins::game::plugin,
            plugins::input::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(plugins::debug::plugin);
    }
}

pub struct AppPlugin2D;

impl Plugin for AppPlugin2D {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            plugins::camera2d::plugin,
            plugins::game2d::plugin,
            plugins::input2d::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(plugins::debug::plugin);
    }
}
