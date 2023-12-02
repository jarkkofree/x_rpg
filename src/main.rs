mod camera;
mod circle;

use bevy::prelude::*;
use camera::CameraPlugin;
use circle::CirclePlugin;

fn main () {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(CirclePlugin)
        .run();
}

