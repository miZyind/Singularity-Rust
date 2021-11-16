use super::AppState;
use crate::{
    constants::{Color, APP_NAME},
    lib::{color::lerp, easing::Function},
};
use bevy::prelude::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Splash)
            .add_system_set(SystemSet::on_enter(AppState::Splash).with_system(setup.system()))
            .add_system_set(SystemSet::on_update(AppState::Splash).with_system(update.system()))
            .add_system_set(SystemSet::on_exit(AppState::Splash).with_system(exit.system()));
    }
}

struct Data {
    title_entity: Entity,
    title_delta: i8,
    title_invert: bool,
}

struct TitleText;
struct TitleImage;
struct AnimationTimer;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle((AnimationTimer, Timer::from_seconds(0.1, true)));

    let title_entity = commands
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
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        APP_NAME,
                        TextStyle {
                            font: assets.load("fonts/Endor.ttf"),
                            font_size: 144.0,
                            color: Color::FOREGROUND_PRIMARY,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(TitleText);

            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(280.), Val::Px(280.)),
                        ..Default::default()
                    },
                    material: materials.add(assets.load("images/logo.png").into()),
                    ..Default::default()
                })
                .insert(TitleImage);
        })
        .id();

    commands.insert_resource(Data {
        title_entity,
        title_delta: 0,
        title_invert: false,
    });
}

fn update(
    time: Res<Time>,
    mut data: ResMut<Data>,
    mut title_text_query: Query<&mut Text, With<TitleText>>,
    // mut title_image_handle_query: Query<&Handle<ColorMaterial>, With<TitleImage>>,
    mut animation_timer_query: Query<&mut Timer, With<AnimationTimer>>,
) {
    for mut timer in animation_timer_query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            if data.title_invert {
                data.title_delta -= 1;

                if data.title_delta == 0 {
                    data.title_invert = false;
                }
            } else {
                data.title_delta += 1;

                if data.title_delta == 10 {
                    data.title_invert = true;
                }
            }

            for mut text in title_text_query.iter_mut() {
                text.sections[0].style.color = lerp(
                    Color::FOREGROUND_PRIMARY,
                    Color::FOREGROUND_SECONDARY,
                    Function::QuadraticInOut(data.title_delta as f32 * 0.1),
                );
            }

            // for handle in title_image_handle_query.iter_mut() {}
        }
    }
}

fn exit(mut commands: Commands, data: Res<Data>) {
    commands.entity(data.title_entity).despawn_recursive();
}
