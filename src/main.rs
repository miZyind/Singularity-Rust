mod bundles;
mod camera;
mod constants;
mod lib;
mod player;
mod resources;
mod state;
mod world;

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use lib::{diagnostics::DiagnosticsPlugin, rapier::RapierPlugin};
use resources::Global;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: constants::APP_NAME.to_string(),
        width: 1600.0,
        height: 900.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .init_resource::<Global>()
    .add_plugins(state::Plugins)
    .add_plugin(DiagnosticsPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_system(exit_on_esc_system);
    app.run();
}
