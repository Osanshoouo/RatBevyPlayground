use bevy::prelude::*;

use first_game::AppPlugin;
use first_game::AppPlugin2D;

fn main() {
    App::new().add_plugins(AppPlugin2D).run();
}
