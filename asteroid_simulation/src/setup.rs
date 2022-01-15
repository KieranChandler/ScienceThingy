use bevy::prelude::*;

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Orbiter
{
    orbit_pos: f32,
}


pub fn setup_asteroid_simulation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // planet
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.5, subdivisions: 5 })),
            material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Planet);

    // orbiter
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.2, subdivisions: 5 })),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.9, 0.5, 0.5),
            ..Default::default()
        })
        .insert(Orbiter { orbit_pos: 0.0 });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

pub fn orbit(mut transform_query: Query<(&mut Transform, &mut Orbiter)>) {
    for (mut transform, mut orbiter) in transform_query.iter_mut() {
        orbiter.orbit_pos = orbiter.orbit_pos + 0.1;
        transform.translation = Vec3::new(
            (orbiter.orbit_pos + 0.5).sin() * 2.0,
            orbiter.orbit_pos.cos() * 2.0,
            0.0,
        );
    }
}
