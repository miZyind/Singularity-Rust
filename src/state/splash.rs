use super::AppState;
use crate::{
    constants::Color,
    lib::{color::lerp, easing::Function},
};
use bevy::{asset::LoadState, prelude::*};

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Handles>()
            .add_state(AppState::Splash)
            .add_system_set(SystemSet::on_enter(AppState::Splash).with_system(enter.system()))
            .add_system_set(SystemSet::on_update(AppState::Splash).with_system(update.system()))
            .add_system_set(SystemSet::on_exit(AppState::Splash).with_system(exit.system()));
    }
}

#[derive(Default)]
struct Handles(Vec<HandleUntyped>);
struct Background(Entity);

fn enter(
    assets: Res<AssetServer>,
    mut handles: ResMut<Handles>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let logo: Handle<Texture> = assets.load("fonts/Endor.ttf");
    let font: Handle<Font> = assets.load("images/logo.png");
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
            material: materials.add(Color::BACKGROUND_TRANSPARENT.into()),
            ..Default::default()
        })
        .insert(Timer::from_seconds(2.0, false))
        .id();

    handles.0.push(logo.clone_untyped());
    handles.0.push(font.clone_untyped());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(Background(entity));
}

fn update(
    assets: Res<AssetServer>,
    handles: Res<Handles>,
    time: Res<Time>,
    mut query: Query<(&mut Timer, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<bevy::ecs::schedule::State<AppState>>,
) {
    if let LoadState::Loaded = assets.get_group_load_state(handles.0.iter().map(|handle| handle.id))
    {
        if let Ok((mut timer, handle)) = query.single_mut() {
            timer.tick(time.delta());
            materials
                .get_mut(handle)
                .unwrap()
                .color
                .set_a(Function::apply(Function::QuadraticIn(timer.percent())));

            if timer.finished() {
                state.set(AppState::Loading).unwrap();
            }
        }
    }
}

fn exit(mut commands: Commands, data: Res<Background>) {
    commands.entity(data.0).despawn_recursive();
    commands.remove_resource::<Handles>();
}
