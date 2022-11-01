use bevy::app::{PluginGroup, PluginGroupBuilder};

mod in_game;
mod loading;
mod main_menu;
mod splash;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Splash,
    Loading,
    MainMenu,
    InGame,
}

pub struct Plugins;
impl PluginGroup for Plugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(splash::State);
        group.add(loading::State);
        group.add(main_menu::State);
        group.add(in_game::State);
    }
}
