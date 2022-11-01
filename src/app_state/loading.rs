use super::AppState;
use crate::{
    assets::UIAssets,
    constants::{APP_NAME, COLOR},
    lib::font::normalize,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_loading_state(LoadingState::new(AppState::Loading).with_collection::<UIAssets>())
            .add_plugin(ProgressPlugin::new(AppState::Loading).continue_to(AppState::MainMenu))
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(enter))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(update))
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit));
    }
}

#[derive(Component)]
struct MainUI;
#[derive(Component)]
struct ProgressBar;

fn enter(mut commands: Commands, resources: Res<UIAssets>, windows: Res<Windows>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: COLOR::BACKGROUND.into(),
            ..default()
        })
        .insert(MainUI)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Percent(10.0),
                            ..default()
                        },
                        size: Size::new(Val::Percent(60.0), Val::Percent(2.0)),
                        ..default()
                    },
                    color: COLOR::INACTIVE.into(),
                    ..default()
                })
                .with_children(|bar| {
                    bar.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                            ..default()
                        },
                        color: COLOR::INFO.into(),
                        ..default()
                    })
                    .insert(ProgressBar);
                });
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    APP_NAME,
                    TextStyle {
                        font: resources.font.clone(),
                        font_size: normalize(&windows, 144.0),
                        color: COLOR::FOREGROUND_PRIMARY,
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
                    size: Size::new(Val::Auto, Val::Percent(30.0)),
                    ..default()
                },
                image: resources.logo.clone().into(),
                ..default()
            });
        });
}

fn update(counter: Res<ProgressCounter>, mut query: Query<&mut Style, With<ProgressBar>>) {
    let progress = counter.progress();
    let mut style = query.single_mut();

    style.size.width = Val::Percent(progress.done as f32 / progress.total as f32 * 100.0);
}

fn exit(mut commands: Commands, query: Query<Entity, With<MainUI>>) {
    let entity = query.single();

    commands.entity(entity).despawn_recursive();
}
