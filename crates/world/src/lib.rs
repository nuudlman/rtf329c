use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create daylight
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add floor
    let floor_raw = Plane3d::new(Vec3::Y, Vec2::splat(10000.0));
    let floor_mesh = floor_raw.mesh().build();
    let floor = meshes.add(floor_mesh.clone());
    let material = materials.add(Color::WHITE);
    commands.spawn((
        Collider::from_bevy_mesh(&floor_mesh, &ComputedColliderShape::TriMesh).unwrap(),
        RigidBody::Fixed,
        MaterialMeshBundle {
            mesh: floor,
            material: material.clone(),
            ..default()
        },
    ));
}
