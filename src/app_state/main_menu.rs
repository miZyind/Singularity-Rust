use super::AppState;
use crate::{
    assets::UIAssets,
    constants::{APP_NAME, COLOR},
    lib::font::normalize,
};
use bevy::prelude::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(enter));
    }
}

fn enter(mut commands: Commands, resources: Res<UIAssets>, windows: Res<Windows>) {
    let background_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: COLOR::BLACK.into(),
            ..default()
        })
        .id();
    let menu_background_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect {
                    top: Val::Percent(5.0),
                    ..default()
                },
                size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                ..default()
            },
            color: COLOR::BACKGROUND.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    "v2022-05-12",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 38.0),
                        ..default()
                    },
                ),
                ..default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    "Quit",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 44.0),
                        color: COLOR::FOREGROUND_SECONDARY,
                        ..default()
                    },
                ),
                ..default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    "Settings",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 44.0),
                        color: COLOR::FOREGROUND_SECONDARY,
                        ..default()
                    },
                ),
                ..default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    "Continue",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 44.0),
                        color: COLOR::FOREGROUND_SECONDARY,
                        ..default()
                    },
                ),
                ..default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    "New",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 44.0),
                        color: COLOR::FOREGROUND_SECONDARY,
                        ..default()
                    },
                ),
                ..default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(25.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    APP_NAME,
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 116.0),
                        color: COLOR::FOREGROUND_SECONDARY,
                    },
                )
                .with_alignment(TextAlignment::CENTER),
                ..default()
            });
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(1.0),
                        ..default()
                    },
                    size: Size::new(Val::Auto, Val::Percent(116.0 / 4.8)),
                    ..default()
                },
                image: resources.logo.clone().into(),
                ..default()
            });
        })
        .id();
    commands
        .entity(background_entity)
        .push_children(&[menu_background_entity]);
    commands.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        color: COLOR::BLACK.into(),
        ..default()
    });
}
