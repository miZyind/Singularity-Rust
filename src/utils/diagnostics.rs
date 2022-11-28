use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::close_on_esc,
};

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup)
            .add_system(update)
            .add_system(close_on_esc);
    }
}

#[derive(Component)]
struct FPSText;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS:".to_string(),
                        style: TextStyle {
                            font: assets.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 12.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: assets.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 12.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        },
        FPSText,
    ));
}

fn update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FPSText>>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            query.single_mut().sections[1].value = format!("{:.0}", average);
        }
    }
}
