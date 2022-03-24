use super::AppState;
use crate::{
    constants::{Theme, FONT_PATH, LOGO_PATH},
    lib::easing::Function,
};
use bevy::{asset::LoadState, prelude::*};

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.init_resource::<Handles>()
            .add_state(AppState::Splash)
            .add_system_set(SystemSet::on_enter(AppState::Splash).with_system(enter))
            .add_system_set(SystemSet::on_update(AppState::Splash).with_system(update))
            .add_system_set(SystemSet::on_exit(AppState::Splash).with_system(exit));
    }
}

#[derive(Default)]
struct Handles(Vec<HandleUntyped>);
struct Background(Entity);

fn enter(mut commands: Commands, assets: Res<AssetServer>, mut hs: ResMut<Handles>) {
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
            color: Theme::BACKGROUND_TRANSPARENT.into(),
            ..Default::default()
        })
        .insert(Timer::from_seconds(2.0, false))
        .id();
    hs.0.push((assets.load(FONT_PATH) as Handle<Font>).clone_untyped());
    hs.0.push((assets.load(LOGO_PATH) as Handle<Image>).clone_untyped());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(Background(entity));
}

fn update(
    assets: Res<AssetServer>,
    handles: Res<Handles>,
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut UiColor)>,
    mut state: ResMut<bevy::ecs::schedule::State<AppState>>,
) {
    if let LoadState::Loaded = assets.get_group_load_state(handles.0.iter().map(|handle| handle.id))
    {
        if let Ok((mut timer, mut color)) = query.get_single_mut() {
            timer.tick(time.delta());
            color
                .0
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
    commands.remove_resource::<Background>();
}
