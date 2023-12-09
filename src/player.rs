use bevy::{
    prelude::*,
    input::mouse::MouseMotion,
    window::CursorGrabMode,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (look, walk));
    }
}

const LOOK_SENSITIVITY: f32 = 0.002;
const WALK_SENSITIVITY: f32 = 0.02;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn look(
    mut q: Query<&mut Transform, With<Camera>>,
    mut er_mm: EventReader<MouseMotion>,
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if mouse.just_pressed(MouseButton::Right) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }

    if window.cursor.grab_mode == CursorGrabMode::Locked {
        for ev in er_mm.read() {
            for mut t in q.iter_mut() {
                let delta_x = -ev.delta.x * LOOK_SENSITIVITY;
                let delta_y = -ev.delta.y * LOOK_SENSITIVITY;

                // Local pitch rotation (up/down)
                let pitch_quat = Quat::from_rotation_x(delta_y);

                // Global yaw rotation (left/right)
                let yaw_quat = Quat::from_rotation_y(delta_x);

                // Apply pitch rotation locally
                t.rotation = t.rotation * pitch_quat;

                // Apply yaw rotation globally
                // This ensures yaw is always relative to the world's up axis
                t.rotation = yaw_quat * t.rotation;
            }
        }
    }
}



fn walk(
    mut q: Query<&mut Transform, With<Camera>>,
    keys: ResMut<Input<KeyCode>>,
) {
    for mut transform in q.iter_mut() {
        let mut move_direction = Vec3::ZERO;
        let forward_direction = transform.rotation.normalize() * Vec3::Z;

        if keys.pressed(KeyCode::W) {
            move_direction += -forward_direction;
        }
        if keys.pressed(KeyCode::S) {
            move_direction += forward_direction;
        }
        if keys.pressed(KeyCode::A) {
            move_direction += forward_direction.cross(Vec3::Y).normalize();
        }
        if keys.pressed(KeyCode::D) {
            move_direction += -forward_direction.cross(Vec3::Y).normalize();
        }

        // Normalize the movement direction to avoid diagonal movement being faster
        if move_direction.length() > 0.0 {
            move_direction = move_direction.normalize() * WALK_SENSITIVITY;
        }

        transform.translation += move_direction;
    }
}
