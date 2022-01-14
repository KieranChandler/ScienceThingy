use bevy::prelude::*;
use bevy::app::*;
use std::time::Duration;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    // asteroid
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.5, subdivisions: 5 })),
        material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
            
    });
    // orbiter
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.2, subdivisions: 5})),
        material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        transform: Transform::from_xyz(0.9, 0.5, 0.5),
        ..Default::default()
    });

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

fn tick() {
    println!("tick");
}
fn main(){
    App::new()
        .insert_resource(Msaa {samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ScheduleRunnerPlugin {
            RunMode: RunMode::Loop { wait: Some(Duration::from_secs(1)) }
        })
        .add_startup_system(setup)
        .run();
}
