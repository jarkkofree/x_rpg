use bevy::{
    prelude::*,
    window::close_on_esc,
};

mod player; // Import the player module
mod gizmos;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugins(player::PlayerPlugin) // Use the PlayerPlugin
        .add_plugins(gizmos::GizmoPlugin)
        .add_systems(Update, close_on_esc)
        .run();
}
