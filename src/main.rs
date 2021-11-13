mod camera;
mod constants;
mod lib;
mod player;
mod world;

#[cfg(feature = "native")]
use bevy::input::system::exit_on_esc_system;

use bevy::prelude::*;
use bevy_rapier3d::{physics::TimestepMode, prelude::*};
use lib::rapier::RapierDynamicForceControllerPlugin;

fn main() {
    let mut app = App::build();

    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Singularity".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::InterpolatedTimestep,
            ..Default::default()
        })
        .add_plugin(RapierDynamicForceControllerPlugin);

    #[cfg(feature = "native")]
    app.add_system(exit_on_esc_system.system());

    #[cfg(feature = "web")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(world::spawn.system())
        .add_startup_system(player::spawn.system())
        .add_startup_system(camera::spawn.system())
        .add_system(camera::zoom.system())
        .run();
}
