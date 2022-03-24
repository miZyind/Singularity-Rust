use super::AppState;
use crate::{
    constants::{Theme, APP_NAME},
    lib::{color::lerp, easing::Function, font::normalize},
    resources::Global,
};
use bevy::prelude::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(enter))
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(update_alpha)
                    .with_system(update_text_color)
                    .with_system(update_image_transform)
                    .with_system(update_progress_bar)
                    .with_system(update_state),
            )
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit));
    }
}

struct Data {
    entity: Entity,
    progress: f32,
    faded_in: bool,
}
#[derive(Component)]
struct Background;
#[derive(Component)]
struct Title;
#[derive(Component)]
struct Image;
#[derive(Component)]
struct ProgressBar;
#[derive(Component)]
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
            color: Theme::BACKGROUND.into(),
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
                    color: Theme::INACTIVE.into(),
                    ..Default::default()
                })
                .with_children(|bar| {
                    bar.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        color: Theme::INFO.into(),
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
                            color: Theme::FOREGROUND_PRIMARY,
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
                    image: resources.logo.clone().into(),
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
                    color: Theme::BLACK_TRANSPARENT.into(),
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
    mut queries: QuerySet<(
        QueryState<&mut UiColor, With<Opaque>>,
        QueryState<&mut UiColor, With<Image>>,
    )>,
    mut text_query: Query<&mut Text>,
) {
    let (mut timer, children) = background_query.single_mut();
    if !timer.finished() {
        timer.tick(time.delta());
        let alpha = Function::apply(Function::QuadraticIn(timer.percent()));
        for child in children.iter() {
            if data.faded_in {
                if let Ok(mut color) = queries.q0().get_mut(*child) {
                    color.0.set_a(alpha);
                }
            } else {
                if let Ok(mut color) = queries.q1().get_mut(*child) {
                    color.0.set_a(alpha);
                }
                if let Ok(mut text) = text_query.get_mut(*child) {
                    text.sections[0].style.color.set_a(alpha);
                }
            }
        }
    }
}

fn update_text_color(
    time: Res<Time>,
    data: Res<Data>,
    mut queries: QuerySet<(
        QueryState<&Timer, With<Background>>,
        QueryState<(&mut Timer, &mut Text), With<Title>>,
    )>,
) {
    if !data.faded_in {
        if queries.q0().single().finished() {
            let mut query = queries.q1();
            let (mut timer, mut text) = query.single_mut();
            timer.tick(time.delta());
            text.sections[0].style.color = lerp(
                Theme::FOREGROUND_PRIMARY,
                Theme::FOREGROUND_SECONDARY,
                Function::QuadraticInOut(if timer.percent() < 0.5 {
                    timer.percent()
                } else {
                    timer.percent_left()
                }),
            );
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
    } else {
        let mut timer = timer_query.single_mut();
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
    if data.faded_in && query.single().just_finished() {
        state.set(AppState::Menu).unwrap();
    }
}

fn exit(mut commands: Commands, data: Res<Data>) {
    commands.entity(data.entity).despawn_recursive();
    commands.remove_resource::<Data>();
}
