use bevy::app::{PluginGroup, PluginGroupBuilder};

mod loading;
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
    }
}
