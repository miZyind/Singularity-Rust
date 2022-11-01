use super::AppState;
use crate::{assets::UIAssets, camera};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Splash)
            .add_loading_state(LoadingState::new(AppState::Splash).with_collection::<UIAssets>())
            .add_plugin(ProgressPlugin::new(AppState::Splash).continue_to(AppState::Loading))
            .add_system_set(SystemSet::on_enter(AppState::Splash).with_system(camera::spawn));
    }
}
