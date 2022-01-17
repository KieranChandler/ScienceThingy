use bevy::prelude::*;
use rand::Rng;

fn neutron_color() -> Color {
    Color::rgb(0.0, 0.7, 1.0)
}

fn should_decay() -> bool {
    rand::thread_rng().gen_range(0..=100) == 0
}

fn uranium_color() -> Color {
    Color::rgb(0.341, 0.615, 0.168)
}

#[derive(Component)]
pub struct Fissile;

#[derive(Component)]
pub struct Neutron {
    direction: Vec3,
    speed: f32,
}

pub fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_uranium)
        .add_system(decay_system)
        .add_system(neutron_movement_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn spawn_uranium(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -215.0, 0.0),
                scale: Vec3::new(5.0, 5.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: uranium_color(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Fissile);
}

pub fn decay_system(mut commands: Commands, fissiles: Query<(&Transform, &Sprite), With<Fissile>>) {
    for fissile in fissiles.iter() {
        if should_decay()
        {
            let (transform, _) = fissile;
            println!("Decay! {}", transform.translation);
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: transform.translation,
                        scale: Vec3::new(5.0, 5.0, 0.0),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: neutron_color(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Neutron {
                    direction: Vec3::new(
                        rand::thread_rng().gen_range(-1.0..=1.0),
                        rand::thread_rng().gen_range(-1.0..=1.0),
                        0.0, // 2D simulation, no Z speed required
                    ),
                    speed: rand::thread_rng().gen_range(1.0..=10.0),
                });
        }
    }
}

pub fn neutron_movement_system(mut neutrons: Query<(&mut Transform, &mut Neutron)>) {
    for (mut transform, neutron) in neutrons.iter_mut() {
        transform.translation = Vec3::new(
            transform.translation.x + (neutron.direction.x * neutron.speed),
            transform.translation.y + (neutron.direction.y * neutron.speed),
            transform.translation.z + (neutron.direction.z * neutron.speed),
        );
    }
}