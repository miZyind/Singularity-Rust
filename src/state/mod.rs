use bevy::app::{PluginGroup, PluginGroupBuilder};

mod in_game;
mod loading;
mod menu;
mod splash;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Splash,
    Loading,
    Menu,
    InGame,
}

pub struct Plugins;
impl PluginGroup for Plugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(splash::State);
        group.add(loading::State);
        group.add(menu::State);
        group.add(in_game::State);
    }
}
