mod camera;
mod constants;
mod lib;
mod player;
mod world;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_rapier3d::{physics::TimestepMode, prelude::*};
use lib::rapier::RapierDynamicForceControllerPlugin;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Singularity".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::InterpolatedTimestep,
            ..Default::default()
        })
        .add_plugin(RapierDynamicForceControllerPlugin)
        .add_system(exit_on_esc_system.system())
        .add_startup_system(world::spawn.system())
        .add_startup_system(player::spawn.system())
        .add_startup_system(camera::spawn.system())
        .add_system(camera::zoom.system())
        .run();
}
