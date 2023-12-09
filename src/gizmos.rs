use bevy::prelude::*;

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, system);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    // Sphere
    commands.spawn(SceneBundle {
        scene: asset_server.load("sphere.glb#Scene0"), // Ensure you have a sphere model in your assets
        transform: Transform::from_xyz(0.0,0.0,0.0),
        ..Default::default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // example instructions
    commands.spawn(
        TextBundle::from_section(
            "sample text",
            TextStyle {
                font_size: 20.,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

// xp, xn, yp, yn, zp, zn
const START: [Vec3; 6] = [Vec3::ZERO; 6];
const VECTOR: [Vec3; 6] = [
    Vec3::new(100.0, 0.0, 0.0),
    Vec3::new(-100.0, 0.0, 0.0),
    Vec3::new(0.0, 100.0, 0.0),
    Vec3::new(0.0, -100.0, 0.0),
    Vec3::new(0.0, 0.0, 100.0),
    Vec3::new(0.0, 0.0, -100.0),
];
const COLOR: [Color; 6] = [
    Color::RED,
    Color::MAROON,
    Color::GREEN,
    Color::DARK_GREEN,
    Color::BLUE,
    Color::NAVY,
];

fn system(
    mut gizmos: Gizmos,
    q: Query<&Transform, With<Camera>>,
) {
    let tr = q.single();

    // Determine the forward direction of the camera
    let forward_direction = tr.forward();

    // Create the end point of the ray, 100.0 units out in the forward direction
    let end_point = tr.translation + forward_direction * 100.0;

    // Draw the ray from the camera's position to the end point
    gizmos.ray(
        tr.translation + Vec3::new(0.,-0.001,0.), // start point is the camera's position
        end_point,       // end point calculated above
        Color::BLACK,    // color of the ray
    );

    for i in 0..6 {
        gizmos.ray(
            START[i],
            VECTOR[i],
            COLOR[i],
        );
    }

    gizmos
        .sphere(Vec3::ZERO, Quat::IDENTITY, 100.0, Color::BLACK);
}