mod camera;
mod constants;
mod lib;
mod player;
mod state;
mod world;

#[cfg(feature = "native")]
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
#[cfg(all(feature = "native", feature = "debug"))]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use lib::{diagnostics::DiagnosticsPlugin, rapier::RapierPlugin};

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        title: constants::APP_NAME.to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(state::Plugins)
    .add_plugin(DiagnosticsPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierPlugin);

    #[cfg(all(feature = "native", feature = "debug"))]
    app.add_plugin(WorldInspectorPlugin::new());

    #[cfg(feature = "web")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    #[cfg(feature = "native")]
    app.add_system(exit_on_esc_system.system());

    app.run();
}
