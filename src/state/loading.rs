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
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(update_alpha.system())
                    .with_system(update_text_color.system())
                    .with_system(update_image_transform.system())
                    .with_system(update_progress_bar.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit.system()));
    }
}

struct Data {
    entity: Entity,
    progress: f32,
}
struct Background;
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
                .insert(Title)
                .insert(Timer::from_seconds(2.0, true));

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
        .insert(Timer::from_seconds(2.0, false))
        .insert(Background)
        .id();

    commands.insert_resource(Data {
        entity,
        progress: 0.0,
    })
}

fn update_alpha(
    time: Res<Time>,
    mut background_query: Query<(&mut Timer, &Children), With<Background>>,
    image_query: Query<&Handle<ColorMaterial>, With<Image>>,
    mut text_query: Query<&mut Text>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((mut timer, children)) = background_query.single_mut() {
        timer.tick(time.delta());

        for child in children.iter() {
            let alpha = Function::apply(Function::QuadraticIn(timer.percent()));

            if let Ok(handle) = image_query.get(*child) {
                materials.get_mut(handle).unwrap().color.set_a(alpha);
            }

            if let Ok(mut text) = text_query.get_mut(*child) {
                text.sections[0].style.color.set_a(alpha);
            }
        }
    }
}

fn update_text_color(
    time: Res<Time>,
    mut query: QuerySet<(
        Query<&Timer, With<Background>>,
        Query<(&mut Timer, &mut Text), With<Title>>,
    )>,
) {
    if let Ok(timer) = query.q0().single() {
        if timer.finished() {
            if let Ok((mut timer, mut text)) = query.q1_mut().single_mut() {
                timer.tick(time.delta());
                text.sections[0].style.color = lerp(
                    Color::FOREGROUND_PRIMARY,
                    Color::FOREGROUND_SECONDARY,
                    Function::QuadraticInOut(if timer.percent() < 0.5 {
                        timer.percent()
                    } else {
                        timer.percent_left()
                    }),
                );
            }
        }
    }
}

fn update_image_transform(time: Res<Time>, mut query: Query<&mut Transform, With<Image>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(
            -std::f32::consts::PI / 60.0 * time.delta_seconds(),
        ));
    }
}

fn update_progress_bar(mut data: ResMut<Data>, mut query: Query<&mut Style, With<ProgressBar>>) {
    if data.progress < 100.0 {
        for mut style in query.iter_mut() {
            data.progress += 0.1;
            style.size.width = Val::Percent(data.progress);
        }
    }
}

fn exit(mut commands: Commands, data: Res<Data>) {
    commands.entity(data.entity).despawn_recursive();
}
