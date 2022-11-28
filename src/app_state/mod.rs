mod in_game;
mod loading;
mod main_menu;
mod splash;

use bevy::app::{PluginGroup, PluginGroupBuilder};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Splash,
    Loading,
    MainMenu,
    InGame,
}

pub struct Plugins;
impl PluginGroup for Plugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(splash::State)
            .add(loading::State)
            .add(main_menu::State)
            .add(in_game::State)
    }
}
