use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    sprite::Mesh2dHandle,
};

pub struct CirclePlugin;

impl Plugin for CirclePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CircleHandles>()
            .add_systems(Startup, add_circle_system)
            .add_systems(Update,modify_circle_system);
    }
}

#[derive(Resource)]
struct CircleHandles {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

impl Default for CircleHandles {
    fn default() -> Self {
        CircleHandles {
            mesh: Mesh2dHandle(Default::default()),
            material: Default::default()
        }
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

fn add_circle_system (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut circle_handles: ResMut<CircleHandles>,
) {
    let circle = shape::Circle::new(10.0).into();
    println!("circle: {:?}", circle);

    circle_handles.mesh = meshes.add(circle).into();
    println!("mesh: {:?}", circle_handles.mesh);

    let red = ColorMaterial::from(Color::RED);
    println!("red: {:?}", red);

    circle_handles.material = materials.add(red);
    println!("material: {:?}", circle_handles.material);

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    println!("transform: {:?}", transform);

    let e = commands.spawn((
        MaterialMesh2dBundle::<ColorMaterial> {
            mesh: circle_handles.mesh.clone(),
            material: circle_handles.material.clone(),
            transform,
            ..default()
        },
    )).id();
    println!("e: {:?}", e);
}

fn modify_circle_system(
    circle_handles: Res<CircleHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Access the mesh and material using the handles
    if let Some(mesh) = meshes.get_mut(&circle_handles.mesh.0) {
        // Modify the mesh here
    }

    if let Some(material) = materials.get_mut(&circle_handles.material) {
        // Modify the material here
    }
}