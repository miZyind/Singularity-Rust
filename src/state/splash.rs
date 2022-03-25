use super::AppState;
use crate::constants::{FONT_PATH, LOGO_PATH};
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

fn enter(mut commands: Commands, assets: Res<AssetServer>, mut hs: ResMut<Handles>) {
    hs.0.push((assets.load(FONT_PATH) as Handle<Font>).clone_untyped());
    hs.0.push((assets.load(LOGO_PATH) as Handle<Image>).clone_untyped());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn update(
    assets: Res<AssetServer>,
    handles: Res<Handles>,
    mut state: ResMut<bevy::ecs::schedule::State<AppState>>,
) {
    if let LoadState::Loaded = assets.get_group_load_state(handles.0.iter().map(|handle| handle.id))
    {
        state.set(AppState::Loading).unwrap();
    }
}

fn exit(mut commands: Commands) {
    commands.remove_resource::<Handles>();
}
