use super::AppState;
use crate::{bundles::spawn_logo_name_with_margin, lib::font::normalize, resources::Global};
use bevy::prelude::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(enter.system()));
    }
}

struct Data {
    background_entity: Entity,
    menu_background_entity: Entity,
}

fn enter(mut commands: Commands, resources: Res<Global>, windows: Res<Windows>) {
    let background_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            material: resources.colors.black.clone(),
            ..Default::default()
        })
        .id();
    let menu_background_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                padding: Rect {
                    top: Val::Percent(5.0),
                    ..Default::default()
                },
                size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                ..Default::default()
            },
            material: resources.colors.background.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        bottom: Val::Percent(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Quit",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 60.0),
                        ..Default::default()
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        bottom: Val::Percent(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Settings",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 60.0),
                        ..Default::default()
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        bottom: Val::Percent(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Load",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 60.0),
                        ..Default::default()
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        bottom: Val::Percent(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "New",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 60.0),
                        ..Default::default()
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            spawn_logo_name_with_margin(
                parent,
                &resources,
                &windows,
                108.0,
                Rect {
                    bottom: Val::Percent(25.0),
                    ..Default::default()
                },
            );
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "v2021-11-28",
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 30.0),
                        ..Default::default()
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();

    commands
        .entity(background_entity)
        .push_children(&[menu_background_entity]);
    commands.insert_resource(Data {
        background_entity,
        menu_background_entity,
    })
}
