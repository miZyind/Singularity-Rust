use crate::constants::PATH;
use bevy::{prelude::*, text::Font};

#[derive(Component)]
struct MyComponent {}

pub struct Global {
    pub font: Handle<Font>,
    pub logo: Handle<Image>,
}
impl FromWorld for Global {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let assets = world.get_resource_mut::<AssetServer>().unwrap();
        Global {
            font: assets.load(PATH::FONT),
            logo: assets.load(PATH::LOGO),
        }
    }
}
