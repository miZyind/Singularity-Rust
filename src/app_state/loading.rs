use super::AppState;
use crate::{
    constants::{APP_NAME, COLOR},
    lib::font::normalize,
    resources::Global,
};
use bevy::prelude::*;
use bevy_ui_animation::{Animation, Ease, TextColor, TransformRotation, Vars};

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(enter))
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(update_progress_bar)
                    .with_system(update_state),
            )
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit));
    }
}

struct Data {
    entity: Entity,
    progress: f32,
}
#[derive(Component)]
struct ProgressBar;

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
            color: COLOR::BACKGROUND.into(),
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
                    color: COLOR::INACTIVE.into(),
                    ..Default::default()
                })
                .with_children(|bar| {
                    bar.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        color: COLOR::INFO.into(),
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
                            color: COLOR::FOREGROUND_PRIMARY,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(Animation::new(Vars {
                    text_color: Some(TextColor {
                        target: COLOR::FOREGROUND_SECONDARY,
                        section: 0,
                    }),
                    duration: 2.0,
                    ease: Ease::PowerIn,
                    repeat: true,
                    yoyo: true,
                    ..Default::default()
                }));
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
                .insert(Animation::new(Vars {
                    transform_rotation: Some(TransformRotation::z(360.0)),
                    duration: 60.0,
                    ease: Ease::Linear,
                    repeat: true,
                    ..Default::default()
                }));
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: COLOR::BLACK.into(),
                    ..Default::default()
                })
                .insert(Animation::new(Vars {
                    color: Some(UiColor(COLOR::BLACK_TRANSPARENT)),
                    duration: 2.0,
                    ease: Ease::PowerIn,
                    ..Default::default()
                }));
        })
        .id();
    commands.insert_resource(Data {
        entity,
        progress: 0.0,
    })
}

fn update_progress_bar(mut data: ResMut<Data>, mut query: Query<&mut Style, With<ProgressBar>>) {
    if data.progress < 100.0 {
        for mut style in query.iter_mut() {
            data.progress += 0.5;
            style.size.width = Val::Percent(data.progress);
        }
    }
}

fn update_state(data: Res<Data>, mut state: ResMut<bevy::ecs::schedule::State<AppState>>) {
    if data.progress == 100.0 {
        state.set(AppState::Menu).unwrap();
    }
}

fn exit(mut commands: Commands, data: Res<Data>) {
    commands.entity(data.entity).despawn_recursive();
    commands.remove_resource::<Data>();
}
