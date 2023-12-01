use bevy::prelude::*;

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, my_system);
    }
}

#[derive(Bundle)]
struct MyBundle {
    component: MyComponent,
}

#[derive(Component)]
struct MyComponent {
    vec: Vec2,
}

fn my_system(
    
) {
    println!("hello");
}