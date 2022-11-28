mod app_state;
mod assets;
mod camera;
mod constants;
mod player;
mod utils;
mod world;

use bevy::{
    log::{self, LogPlugin},
    prelude::*,
};
#[cfg(feature = "debug")]
use {bevy_inspector_egui::WorldInspectorPlugin, utils::diagnostics::DiagnosticsPlugin};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: constants::APP_NAME.to_string(),
                        width: 1600.0,
                        height: 900.0,
                        // https://github.com/bevyengine/bevy/issues/4869
                        #[cfg(target_arch = "wasm32")]
                        canvas: Some("#singularity".to_string()),
                        #[cfg(target_arch = "wasm32")]
                        fit_canvas_to_parent: true,
                        #[cfg(target_arch = "wasm32")]
                        scale_factor_override: Some(1.0),
                        ..default()
                    },
                    ..default()
                })
                .set(LogPlugin {
                    filter: "error".to_string(),
                    level: log::Level::ERROR,
                }),
        )
        .add_plugins(app_state::Plugins);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(DiagnosticsPlugin);

    app.run();
}
