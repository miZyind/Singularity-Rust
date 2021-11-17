use super::AppState;
use crate::{
    constants::{Color, APP_NAME},
    lib::{color::lerp, easing::Function, font::normalize},
};
use bevy::prelude::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(enter.system()))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(update.system()))
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit.system()));
    }
}

struct Data {
    entity: Entity,
    delta: i8,
    invert: bool,
    progress: f32,
}
struct AnimationTimer;
struct Title;
struct Image;
struct ProgressBar;

fn enter(
    windows: Res<Windows>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
) {
    let entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            material: materials.add(Color::BACKGROUND.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Percent(10.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Percent(60.0), Val::Percent(2.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::INACTIVE.into()),
                    ..Default::default()
                })
                .with_children(|outer_bar| {
                    outer_bar
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            material: materials.add(Color::INFO.into()),
                            ..Default::default()
                        })
                        .insert(ProgressBar);
                });

            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        APP_NAME,
                        TextStyle {
                            font: assets.load("fonts/Endor.ttf"),
                            font_size: normalize(windows, 144),
                            color: Color::FOREGROUND_PRIMARY,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(Title);

            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        margin: Rect {
                            bottom: Val::Percent(1.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Auto, Val::Percent(30.0)),
                        ..Default::default()
                    },
                    material: materials.add(assets.load("images/logo.png").into()),
                    ..Default::default()
                })
                .insert(Image);
        })
        .id();

    commands.insert_resource(Data {
        entity,
        delta: 0,
        invert: false,
        progress: 0.0,
    })
}

fn update(
    time: Res<Time>,
    mut data: ResMut<Data>,
    mut animation_timer_query: Query<&mut Timer, With<AnimationTimer>>,
    mut title_text_query: Query<&mut Text, With<Title>>,
    mut title_image_transform_query: Query<&mut Transform, With<Image>>,
    mut progress_bar_query: Query<&mut Style, With<ProgressBar>>,
) {
    for mut timer in animation_timer_query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            if data.invert {
                data.delta -= 1;

                if data.delta == 0 {
                    data.invert = false;
                }
            } else {
                data.delta += 1;

                if data.delta == 10 {
                    data.invert = true;
                }
            }

            for mut text in title_text_query.iter_mut() {
                text.sections[0].style.color = lerp(
                    Color::FOREGROUND_PRIMARY,
                    Color::FOREGROUND_SECONDARY,
                    Function::QuadraticInOut(data.delta as f32 * 0.1),
                );
            }
        }
    }

    for mut transform in title_image_transform_query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(
            -std::f32::consts::PI / 60.0 * time.delta_seconds(),
        ));
    }

    if data.progress < 50.0 {
        for mut style in progress_bar_query.iter_mut() {
            data.progress += 0.1;
            style.size.width = Val::Percent(data.progress);
        }
    }
}

fn exit(mut commands: Commands, data: Res<Data>) {
    commands.entity(data.entity).despawn_recursive();
}
