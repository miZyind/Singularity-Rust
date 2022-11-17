mod app_state;
mod assets;
mod camera;
mod constants;
mod lib;
mod player;
mod world;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use bevy::{log::LogSettings, prelude::*};

fn main() {
    let mut app = App::new();
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.insert_resource(WindowDescriptor {
        title: constants::APP_NAME.to_string(),
        width: 1600.0,
        height: 900.0,
        ..default()
    })
    .insert_resource(LogSettings {
        filter: "error".into(),
        level: bevy::log::Level::ERROR,
    })
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins)
    .add_plugins(app_state::Plugins)
    .add_system(bevy::window::close_on_esc)
    .run();
}
