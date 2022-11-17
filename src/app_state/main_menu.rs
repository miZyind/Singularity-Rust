use super::AppState;
use crate::{
    assets::UIAssets,
    constants::{APP_NAME, COLOR},
    lib::font::normalize,
};
use bevy::{app::AppExit, prelude::*};

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(enter))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(update)
                    .with_system(handle_start_button)
                    .with_system(handle_exit_button),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(exit));
    }
}

const NORMAL_BUTTON: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const HOVERED_BUTTON: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);
const PRESSED_BUTTON: Color = Color::rgba(0.25, 0.25, 0.25, 1.0);

#[derive(Component)]
struct MainUI;
#[derive(Component)]
pub struct StartButton;
#[derive(Component)]
pub struct ExitButton;

fn enter(mut commands: Commands, resources: Res<UIAssets>, windows: Res<Windows>) {
    let button_size = Size::new(Val::Px(150.0), Val::Px(40.0));
    commands
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
        .insert(MainUI)
        .with_children(|parent| {
            parent
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
                            "v2022-11-17",
                            TextStyle {
                                font: resources.font.clone(),
                                font_size: normalize(&windows, 38.0),
                                ..default()
                            },
                        ),
                        ..default()
                    });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: button_size,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect {
                                    bottom: Val::Percent(10.0),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(ExitButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    "Exit",
                                    TextStyle {
                                        font: resources.font.clone(),
                                        font_size: normalize(&windows, 44.0),
                                        color: COLOR::FOREGROUND_SECONDARY,
                                    },
                                ),
                                ..default()
                            });
                        });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: button_size,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect {
                                    bottom: Val::Percent(10.0),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    "Settings",
                                    TextStyle {
                                        font: resources.font.clone(),
                                        font_size: normalize(&windows, 44.0),
                                        color: COLOR::FOREGROUND_SECONDARY,
                                    },
                                ),
                                ..default()
                            });
                        });
                    // parent
                    //     .spawn_bundle(ButtonBundle {
                    //         style: Style {
                    //             size: button_size,
                    //             justify_content: JustifyContent::Center,
                    //             align_items: AlignItems::Center,
                    //             margin: UiRect {
                    //                 bottom: Val::Percent(10.0),
                    //                 ..default()
                    //             },
                    //             ..default()
                    //         },
                    //         ..default()
                    //     })
                    //     .with_children(|parent| {
                    //         parent.spawn_bundle(TextBundle {
                    //             text: Text::from_section(
                    //                 "Continue",
                    //                 TextStyle {
                    //                     font: resources.font.clone(),
                    //                     font_size: normalize(&windows, 44.0),
                    //                     color: COLOR::FOREGROUND_SECONDARY,
                    //                 },
                    //             ),
                    //             ..default()
                    //         });
                    //     });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: button_size,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect {
                                    bottom: Val::Percent(10.0),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(StartButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    "New",
                                    TextStyle {
                                        font: resources.font.clone(),
                                        font_size: normalize(&windows, 44.0),
                                        color: COLOR::FOREGROUND_SECONDARY,
                                    },
                                ),
                                ..default()
                            });
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
                });
        });
}

fn update(mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
        }
    }
}

fn handle_start_button(
    query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut state: ResMut<bevy::ecs::schedule::State<AppState>>,
) {
    for interaction in query.iter() {
        if Interaction::Clicked == *interaction {
            state.set(AppState::InGame).unwrap();
        }
    }
}

fn handle_exit_button(
    query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut exit: ResMut<Events<AppExit>>,
) {
    for interaction in query.iter() {
        if Interaction::Clicked == *interaction {
            exit.send(AppExit);
        }
    }
}

fn exit(mut commands: Commands, query: Query<Entity, With<MainUI>>) {
    commands.entity(query.single()).despawn_recursive();
}
