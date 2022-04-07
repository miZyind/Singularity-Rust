mod app_state;
mod camera;
mod constants;
mod lib;
mod player;
mod resources;
mod world;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_ui_animation::*;
use resources::Global;

fn main() {
    let mut app = App::new();
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.insert_resource(WindowDescriptor {
        title: constants::APP_NAME.to_string(),
        width: 1600.0,
        height: 900.0,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins)
    .add_plugins(app_state::Plugins)
    .add_plugin(AnimationPlugin)
    .init_resource::<Global>()
    .add_system(exit_on_esc_system)
    .run();
}
