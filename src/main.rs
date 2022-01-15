use bevy::prelude::*;

use asteroid_simulation::setup::setup_asteroid_simulation;
use asteroid_simulation::setup::orbit;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_asteroid_simulation)
        .add_system(orbit)
        .run();
}
