use crate::{camera, player, world};

use super::AppState;
use bevy::prelude::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(world::spawn)
                .with_system(player::spawn),
        )
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(camera::zoom));
    }
}
