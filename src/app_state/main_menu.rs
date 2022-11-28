use super::AppState;
use crate::{
    assets::UIAssets,
    constants::{Color, APP_NAME},
    utils::font::normalize,
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

#[derive(Component)]
struct MainUI;
#[derive(Component)]
pub struct StartButton;
#[derive(Component)]
pub struct ExitButton;

fn enter(mut commands: Commands, resources: Res<UIAssets>, windows: Res<Windows>) {
    let button_size = Size::new(Val::Px(150.0), Val::Px(40.0));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            MainUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect {
                            top: Val::Percent(5.0),
                            ..default()
                        },
                        size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::BACKGROUND.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            margin: UiRect {
                                top: Val::Percent(1.0),
                                ..default()
                            },
                            size: Size::new(Val::Auto, Val::Percent(116.0 / 4.8)),
                            ..default()
                        },
                        image: resources.logo.clone().into(),
                        ..default()
                    });
                    parent.spawn(TextBundle {
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
                                color: Color::FOREGROUND_SECONDARY,
                            },
                        )
                        .with_alignment(TextAlignment::CENTER),
                        ..default()
                    });
                    parent
                        .spawn((
                            ButtonBundle {
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
                            },
                            StartButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "New",
                                    TextStyle {
                                        font: resources.font.clone(),
                                        font_size: normalize(&windows, 44.0),
                                        color: Color::FOREGROUND_SECONDARY,
                                    },
                                ),
                                ..default()
                            });
                        });
                    parent
                        .spawn(ButtonBundle {
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
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Settings",
                                    TextStyle {
                                        font: resources.font.clone(),
                                        font_size: normalize(&windows, 44.0),
                                        color: Color::FOREGROUND_SECONDARY,
                                    },
                                ),
                                ..default()
                            });
                        });
                    parent
                        .spawn((
                            ButtonBundle {
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
                            },
                            ExitButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Exit",
                                    TextStyle {
                                        font: resources.font.clone(),
                                        font_size: normalize(&windows, 44.0),
                                        color: Color::FOREGROUND_SECONDARY,
                                    },
                                ),
                                ..default()
                            });
                        });
                    parent.spawn(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                bottom: Val::Px(0.0),
                                ..default()
                            },
                            ..default()
                        },
                        text: Text::from_section(
                            "v2022-11-28",
                            TextStyle {
                                font: resources.font.clone(),
                                font_size: normalize(&windows, 38.0),
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn update(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::None => {
                *color = Color::BLACK_TRANSPARENT.into();
            }
            Interaction::Hovered => {
                *color = Color::HOVERED_BUTTON.into();
            }
            Interaction::Clicked => {
                *color = Color::PRESSED_BUTTON.into();
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
