use super::AppState;
use crate::{
    constants::{Color, APP_NAME},
    lib::{color::lerp, easing::Function, font::normalize},
    resources::Global,
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
                    .with_system(update_progress_bar.system())
                    .with_system(update_state.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit.system()));
    }
}

struct Data {
    entity: Entity,
    progress: f32,
    faded_in: bool,
}
struct Background;
struct Title;
struct Image;
struct ProgressBar;
struct Opaque;

const FADE_DURATION: f32 = 1.0;
const BLINK_DURATION: f32 = 2.0;

fn enter(mut commands: Commands, resources: Res<Global>, windows: Res<Windows>) {
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
            material: resources.colors.background.clone(),
            ..Default::default()
        })
        .insert(Timer::from_seconds(FADE_DURATION, false))
        .insert(Background)
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
                    material: resources.colors.inactive.clone(),
                    ..Default::default()
                })
                .with_children(|bar| {
                    bar.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        material: resources.colors.info.clone(),
                        ..Default::default()
                    })
                    .insert(ProgressBar);
                });
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        APP_NAME,
                        TextStyle {
                            font: resources.font.clone(),
                            font_size: normalize(&windows, 144.0),
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
                .insert(Timer::from_seconds(BLINK_DURATION, true));
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
                    material: resources.logo.clone(),
                    ..Default::default()
                })
                .insert(Image);
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    material: resources.colors.black_transparent.clone(),
                    ..Default::default()
                })
                .insert(Opaque);
        })
        .id();
    commands.insert_resource(Data {
        entity,
        progress: 0.0,
        faded_in: false,
    })
}

fn update_alpha(
    time: Res<Time>,
    data: Res<Data>,
    mut background_query: Query<(&mut Timer, &Children), With<Background>>,
    opaque_query: Query<&Handle<ColorMaterial>, With<Opaque>>,
    image_query: Query<&Handle<ColorMaterial>, With<Image>>,
    mut text_query: Query<&mut Text>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((mut timer, children)) = background_query.single_mut() {
        if !timer.finished() {
            timer.tick(time.delta());
            let alpha = Function::apply(Function::QuadraticIn(timer.percent()));
            for child in children.iter() {
                if data.faded_in {
                    if let Ok(handle) = opaque_query.get(*child) {
                        materials.get_mut(handle).unwrap().color.set_a(alpha);
                    }
                } else {
                    if let Ok(handle) = image_query.get(*child) {
                        materials.get_mut(handle).unwrap().color.set_a(alpha);
                    }

                    if let Ok(mut text) = text_query.get_mut(*child) {
                        text.sections[0].style.color.set_a(alpha);
                    }
                }
            }
        }
    }
}

fn update_text_color(
    time: Res<Time>,
    data: Res<Data>,
    mut query: QuerySet<(
        Query<&Timer, With<Background>>,
        Query<(&mut Timer, &mut Text), With<Title>>,
    )>,
) {
    if !data.faded_in {
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
}

fn update_image_transform(time: Res<Time>, mut query: Query<&mut Transform, With<Image>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(
            -std::f32::consts::PI / 60.0 * time.delta_seconds(),
        ));
    }
}

fn update_progress_bar(
    mut data: ResMut<Data>,
    mut query: Query<&mut Style, With<ProgressBar>>,
    mut timer_query: Query<&mut Timer, With<Background>>,
) {
    if data.progress < 100.0 {
        for mut style in query.iter_mut() {
            data.progress += 0.5;
            style.size.width = Val::Percent(data.progress);
        }
    } else if let Ok(mut timer) = timer_query.single_mut() {
        if !data.faded_in && timer.just_finished() {
            data.faded_in = true;
            timer.reset();
        }
    }
}

fn update_state(
    data: Res<Data>,
    query: Query<&Timer, With<Background>>,
    mut state: ResMut<bevy::ecs::schedule::State<AppState>>,
) {
    if let Ok(timer) = query.single() {
        if data.faded_in && timer.just_finished() {
            state.set(AppState::Menu).unwrap();
        }
    }
}

fn exit(mut commands: Commands, data: Res<Data>) {
    commands.entity(data.entity).despawn_recursive();
    commands.remove_resource::<Data>();
}
