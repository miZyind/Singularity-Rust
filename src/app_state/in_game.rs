use super::AppState;
use crate::{camera, player, world};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(world::spawn)
                    .with_system(player::spawn),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(camera::zoom)
                    .with_system(player::handle_move),
            );
    }
}
