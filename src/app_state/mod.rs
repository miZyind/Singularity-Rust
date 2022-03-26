use bevy::app::{PluginGroup, PluginGroupBuilder};

mod game;
mod initialization;
mod loading;
mod menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Initialization,
    Loading,
    Menu,
    InGame,
}

pub struct Plugins;
impl PluginGroup for Plugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(initialization::State);
        group.add(loading::State);
        group.add(menu::State);
        group.add(game::State);
    }
}
